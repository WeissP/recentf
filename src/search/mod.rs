mod candidates;
use super::tramp;
use anyhow::{anyhow, Context, Error, Result};
pub use candidates::{Candidate, Candidates, Status};
use std::{collections::HashMap, path::PathBuf, str::FromStr, time::Duration};

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
        todo!()
    }
}

pub struct SearchOption {
    limit: usize,
}

#[derive(Debug, Clone)]
pub struct Fmt {
    pub indent: &'static str,
    pub prefix: String,
    pub width: usize,
    pub sep: &'static str,
    pub tramp: tramp::Prefix,
}

impl Fmt {
    pub fn new(
        indent: &'static str,
        prefix: String,
        width: usize,
        sep: &'static str,
        tramp: tramp::Prefix,
    ) -> Self {
        Self {
            indent,
            prefix,
            width,
            sep,
            tramp,
        }
    }

    // pub fn update_tramp(&mut self, tramp: Tramp) {
    //     self.tramp = tramp;
    //     self.prefix = match self.tramp.tramp_type.as_str() {
    //         "sudo" => "♔ ".to_owned(),
    //         "ssh" => format!("[{}] ", self.tramp.id.0),
    //         _ => String::new(),
    //     }
    // }
}

impl<'a> pathtrie::format::Format for Fmt {
    fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    fn single_indent(&self) -> &str {
        self.indent
    }

    fn max_width(&self) -> usize {
        self.width
    }

    fn suffix(&self, segs: pathtrie::Segs, _level_stack: &Vec<usize>) -> String {
        let sep = self.sep;
        let path = segs.join("/");
        format!("{sep}/{path}")
        // if self.tramp.is_empty() {
        // } else {
        //     let tramp_type = &self.tramp.tramp_type;
        //     let tramp_path = &self.tramp.tramp_path;
        //     format!("{sep}/{tramp_type}:{tramp_path}:/{path}")
        // }
    }
}
