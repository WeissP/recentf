use super::Candidates;
use crate::tramp::{self, AliasMap};
use anyhow::{anyhow, Context, Error, Result};
use once_cell::sync::OnceCell;
use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

pub struct Options<'a> {
    prefix: &'a str,
    alias_map: &'a AliasMap,
}

impl<'a> Options<'a> {
    pub fn new(prefix: &'a str, alias_map: &'a AliasMap) -> Self {
        Self { prefix, alias_map }
    }
}

impl<'a> pathtrie::format::Format for Options<'a> {
    fn prefix(&self) -> &str {
        static PREFIX: OnceCell<String> = OnceCell::new();
        PREFIX.get_or_init(|| {
            tramp::Prefix::from_str(&self.prefix)
                .with_context(|| format!("could not parse tramp prefix: {}", self.prefix))
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
        if !self.prefix.is_empty() {
            res.push_str(self.prefix);
            res.push(':');
        }
        res.push('/');
        res.push_str(&segs.join("/"));
        res
    }
}
