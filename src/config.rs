use crate::{clean::FilterRules, search::SearchOption, tramp::AliasMap};
use anyhow::{Context, Result};
use bimap::BiMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub alias_map: AliasMap,
    pub search_option: SearchOption,
    pub filter_rules: FilterRules,
    pub database: Database,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub url: String,
}

fn config_path() -> PathBuf {
    let mut p = PathBuf::new();
    match env::var("XDG_CONFIG_HOME") {
        Ok(c) => p.push(c),
        Err(_) => {
            let home = env::var("HOME").expect("neither env xdg_config_home nor home is set");
            p.push(home);
            p.push(".config");
        }
    };
    p.push("recentf");
    p.push("recentf.toml");
    p
}

fn try_get_config() -> Result<&'static Config> {
    static CONFIG: OnceCell<Config> = OnceCell::new();
    CONFIG.get_or_try_init(|| {
        let contents = std::fs::read_to_string(config_path())
            .with_context(|| format!("could not read config file with path {:?}", config_path()))?;
        toml::from_str(&contents).context("could not deserialize config")
    })
}

pub fn config() -> &'static Config {
    try_get_config().expect("failed to get config")
}

pub fn database_path() -> &'static str {
    &config().database.url
}
