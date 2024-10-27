use std::{ffi::OsStr, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{database, search::Query};

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

fn try_unwrap(c: &Option<String>) -> Option<&str> {
    match c {
        Some(x) if !x.is_empty() => Some(x),
        _ => None,
    }
}

impl FilterRule {
    fn is_matched(&self, file_path: &str) -> bool {
        let p = Path::new(file_path);
        if let Some(dir_prefix) = try_unwrap(&self.dir_prefix) {
            if !p.starts_with(Path::new(dir_prefix)) {
                return false;
            }
        }

        if let Some(ext) = try_unwrap(&self.ext) {
            if !(p.extension() == Some(OsStr::new(ext))) {
                return false;
            }
        }

        let prefix = try_unwrap(&self.name_prefix);
        let suffix = try_unwrap(&self.name_suffix);
        if prefix.is_some() | suffix.is_some() {
            if let Some(Some(file_name)) = p.file_name().map(|x| x.to_str()) {
                if let Some(name_prefix) = prefix {
                    if !file_name.starts_with(name_prefix) {
                        return false;
                    }
                }
                if let Some(name_suffix) = suffix {
                    if !file_name.ends_with(name_suffix) {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

pub async fn clean(conn: &PgPool) -> Result<()> {
    let cands = database::search(conn, Query::default()).await?;
    if let Some(cands) = cands.get("") {
        for cand in cands {
            if !Path::new(cand.full_path()).exists() {
                database::change_deleted_flag(conn, "", cand.full_path(), true).await?;
            }
        }
    }

    Ok(())
}
