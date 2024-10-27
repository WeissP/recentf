use super::Prefix;

use bimap::BiBTreeMap;
use serde::Deserialize;

pub type Alias = String;
pub type Host = String;

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct AliasMap(BiBTreeMap<Alias, Host>);

impl AliasMap {
    pub fn matched_host<'a, 'b>(&'a self, alias: &'b str) -> Option<&'a str> {
        self.0.iter().find_map(|(k, v)| {
            if k.contains(alias) {
                Some(v.as_str())
            } else {
                None
            }
        })
    }
}

impl Prefix {
    pub fn show_as_aliases<'a, 'b>(&'a self, m: &'b AliasMap) -> String {
        let mut res = String::new();
        if self.0.is_empty() {
            return res;
        }
        res.push('[');
        for p in &self.0 {
            res.push_str(&p.user);
            res.push('|');
            if let Some(alias) = m.0.get_by_right(&p.host) {
                res.push_str(alias);
            } else {
                res.push_str(&p.host);
            }
        }
        res.push(']');
        res
    }
}
