use crate::tramp::{self, AliasMap};
use anyhow::Context;
use once_cell::sync::OnceCell;
use std::str::FromStr;

pub struct Options<'a> {
    tramp_prefix: &'a str,
    alias_map: &'a AliasMap,
}

impl<'a> Options<'a> {
    pub fn new(tramp_prefix: &'a str, alias_map: &'a AliasMap) -> Self {
        Self {
            tramp_prefix,
            alias_map,
        }
    }
}

impl<'a> pathtrie::format::Format for Options<'a> {
    fn prefix(&self) -> &str {
        static PREFIX: OnceCell<String> = OnceCell::new();
        PREFIX.get_or_init(|| {
            tramp::Prefix::from_str(&self.tramp_prefix)
                .with_context(|| format!("could not parse tramp prefix: {}", self.tramp_prefix))
                .unwrap()
                .show_as_aliases(self.alias_map)
        })
    }

    fn single_indent(&self) -> &str {
        "ǁ    "
    }

    fn max_width(&self) -> usize {
        50
    }

    fn suffix(&self, segs: pathtrie::Segs, _level_stack: &Vec<usize>) -> String {
        let mut res = String::new();
        res.push_str("』𝔰𝔢𝔭『");
        if !self.tramp_prefix.is_empty() {
            res.push_str(self.tramp_prefix);
            res.push(':');
        }
        res.push('/');
        res.push_str(&segs.join("/"));
        res
    }
}
