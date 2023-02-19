use std::{ffi::OsStr, path::Path};

use crate::config::config;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct FilterRules(Vec<FilterRule>);

impl FilterRules {
    pub fn is_matched(&self, file_path: &str) -> bool {
        self.0.iter().any(|x| x.is_matched(file_path))
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FilterRule {
    pub dir_prefix: Option<String>,
    pub ext: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
}

impl FilterRule {
    fn is_matched(&self, file_path: &str) -> bool {
        let p = Path::new(file_path);
        if let Some(dir_prefix) = &self.dir_prefix {
            if !p.starts_with(Path::new(dir_prefix)) {
                return false;
            }
        }

        if let Some(ext) = &self.ext {
            if !(p.extension() == Some(OsStr::new(ext))) {
                return false;
            }
        }

        if self.name_prefix.is_some() | self.name_suffix.is_some() {
            if let Some(Some(file_name)) = p.file_name().map(|x| x.to_str()) {
                if let Some(name_prefix) = &self.name_prefix {
                    if !file_name.starts_with(name_prefix) {
                        return false;
                    }
                }
                if let Some(name_suffix) = &self.name_suffix {
                    if !file_name.ends_with(name_suffix) {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

pub fn clean() -> Result<()> {
    todo!()
}
