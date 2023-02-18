use crate::{clean::FilterRules, search::SearchOption, tramp::AliasMap};
use once_cell::sync::OnceCell;
use std::{
    env,
    path::{Path, PathBuf},
};

pub struct Config {
    alias_map: AliasMap,
    search_option: SearchOption,
    filter_rules: FilterRules,
}

// TODO: oncecell
fn root_path() -> &'static str {
    static PATH: OnceCell<&'static str> = OnceCell::new();
    PATH.get_or_init(|| {
        let mut p = PathBuf::new();
        match env::var("XDG_CONFIG_HOME") {
            Ok(c) => p.push(c),
            Err(_) => {
                let mut home =
                    env::var("HOME").expect("neither env xdg_config_home nor home is set");
                p.push(home);
                p.push(".config");
            }
        };
        p.push("recentf");
        todo!()
        // p.to_str().expect("could not get valid root path")
    })
}

pub fn database_path() -> &'static str {
    "postgres://weiss@localhost/recentf"
}
