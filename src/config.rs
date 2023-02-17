use crate::{clean::FilterRules, search::SearchOption, tramp::AliasMap};
use std::{env, path::{PathBuf, Path}};

pub struct Config {
    alias_map: AliasMap,
    search_option: SearchOption,
    filter_rules: FilterRules,
}

// TODO: oncecell
fn root_path() -> PathBuf {
    let mut p = PathBuf::new();
    match env::var("XDG_CONFIG_HOME") {
        Ok(c) => p.push(c),
        Err(_) => {
            let mut home = env::var("HOME").expect("neither env xdg_config_home nor home is set");
            p.push(home);
            p.push(".config");
        }
    };
    p.push("recentf");
    p
}

pub fn database_path() -> &'static str {
    todo!()
}
