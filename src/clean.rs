use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FilterRules(Vec<FilterRule>);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FilterRule {
    pub tramp_id: Option<usize>,
    pub dir_prefix: Option<String>,
    pub ext: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
}

pub fn clean() -> Result<()> {
    todo!()
}
