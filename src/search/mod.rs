mod candidates;
mod format;
use super::tramp;
use crate::config::config;
use anyhow::{Error, Result};
pub use candidates::{Candidate, Candidates, Status};

use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Query {
    pub tramp_aliases: Vec<tramp::Alias>,
    pub paths: Vec<String>,
    pub names: Vec<String>,
}

impl Query {
    pub fn new(tramp_aliases: Vec<tramp::Alias>, paths: Vec<String>, names: Vec<String>) -> Self {
        Self {
            tramp_aliases,
            paths,
            names,
        }
    }
}

impl FromStr for Query {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut res = Self::default();
        let alias_map = &config().alias_map;
        macro_rules! push {
            ($v:expr, $seg:expr) => {
                if !$seg.is_empty() {
                    $v.push($seg.to_string())
                }
            };
        }

        for seg in s.split(' ') {
            let mut chars = seg.chars();
            match chars.next() {
                Some('/') => push!(res.paths, chars.as_str()),
                Some('@') => {
                    let input = chars.as_str();
                    let item = alias_map.matched_host(input).unwrap_or(input);
                    push!(res.tramp_aliases, item)
                }
                _ => push!(res.names, seg),
            };
        }
        Ok(res)
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchOption {
    limit: usize,
}
