use std::{borrow::Cow, path::Path};

use anyhow::Result;
use wild_doc_script::{Vars, WildDocValue};

use super::Parser;

impl Parser {
    pub(super) async fn get_include_content(
        &mut self,
        attr: Vars,
        with_parse: bool,
    ) -> Result<Vec<u8>> {
        if let Some(src) = attr.get("src") {
            let src = src.to_str();
            let (xml, filename) = self
                .include_adaptor
                .lock()
                .include(Path::new(src.as_ref()))
                .map_or_else(
                    || {
                        let mut r = (None, Cow::Borrowed(""));
                        if let Some(substitute) = attr.get("substitute") {
                            let substitute = substitute.to_str();
                            if let Some(xml) = self
                                .include_adaptor
                                .lock()
                                .include(Path::new(substitute.as_ref()))
                            {
                                r = (Some(xml), substitute);
                            }
                        }
                        r
                    },
                    |xml| (Some(xml), src),
                );
            if let Some(xml) = xml {
                if xml.len() > 0 {
                    return Ok(if with_parse {
                        self.include_stack.push(filename.into());
                        let mut pos = 0;
                        let r = self.parse(xml.as_slice(), &mut pos).await?;
                        self.include_stack.pop();
                        r
                    } else {
                        xml.as_ref().clone()
                    });
                }
            }
        }
        Ok(b"".to_vec())
    }
}
