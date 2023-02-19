mod candidates;
mod format;
use super::tramp;
use anyhow::{anyhow, Context, Error, Result};
pub use candidates::{Candidate, Candidates, Status};
use pathtrie::{node::Node, Tree};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, str::FromStr, time::Duration};

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
        for seg in s.split(' ') {
            let mut chars = seg.chars();
            match chars.next() {
                Some('/') => res.paths.push(chars.as_str().to_owned()),
                // Some('@') if seg.len() > 1 => res.tramp_ids = None,
                Some('@') if seg.len() > 1 => todo!(),
                _ => res.names.push(seg.to_owned()),
            };
        }
        Ok(res)
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchOption {
    limit: usize,
}
