use crate::{clean::FilterRules, config::config};
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use pathtrie::Tree;
use std::{cmp::Ordering, collections::HashMap, fmt::Display, path::Path, time::SystemTime};

const DAY: u64 = 60 * 60 * 24;

/// the field order matters because of Ord derivation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate {
    status: Status,
    last_ref: LastRef,
    freq: i32,
    full_path: String,
}

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, Ord)]
#[repr(i32)]
pub enum Status {
    Filtered = 1,
    Normal = 2,
    Favourite = 3,
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (*self as i32).partial_cmp(&(*other as i32))
    }
}

#[derive(Eq, Debug, Ord)]
struct LastRef(u64);

impl PartialEq for LastRef {
    fn eq(&self, other: &Self) -> bool {
        match (self.levels(), other.levels()) {
            (None, None) => self.weeks() == other.weeks(),
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for LastRef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

fn now() -> u64 {
    static NOW: OnceCell<u64> = OnceCell::new();
    NOW.get_or_init(
        || match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        },
    )
    .to_owned()
}

impl LastRef {
    fn days(&self) -> u64 {
        (now() - self.0) / DAY
    }

    fn weeks(&self) -> u64 {
        self.days() / 7
    }

    fn levels(&self) -> Option<usize> {
        let days = self.days();
        if days >= 30 {
            return None;
        }
        Some(match days {
            0 => 0,
            1 => 1,
            2 | 3 => 2,
            4..=6 => 3,
            7..=10 => 4,
            11..=14 => 5,
            15..=19 => 6,
            20..=24 => 7,
            _ => 8,
        })
    }
}

impl Candidate {
    pub fn full_path(&self) -> &str {
        &self.full_path
    }

    pub fn new(full_path: String, last_ref: i64, freq: i32, status: Status) -> Self {
        Self {
            status,
            last_ref: LastRef(
                last_ref
                    .try_into()
                    .expect("freq can not be converted into u64"),
            ),
            freq,
            full_path,
        }
    }

    fn path_segs(&self) -> Vec<&str> {
        Path::new(&self.full_path[1..])
            .iter()
            .map(|x| x.to_str().unwrap())
            .collect()
    }
}

#[derive(Default, Debug)]
pub struct Candidates(HashMap<String, Vec<Candidate>>);

impl Candidates {
    pub fn single(id: String, can: Vec<Candidate>) -> Self {
        let mut hm = HashMap::new();
        hm.insert(id, can);
        Self(hm)
    }

    pub fn insert(&mut self, id: String, cs: Vec<Candidate>) -> Result<()> {
        if let Some(old) = self.0.insert(id, cs) {
            Err(anyhow!(
                "Candidates already has value {:?} while inserting new",
                old,
            ))
        } else {
            Ok(())
        }
    }

    pub fn get(&self, id: &str) -> Option<&Vec<Candidate>> {
        self.0.get(id)
    }

    fn sort(&mut self) -> () {
        for (_, cs) in self.0.iter_mut() {
            // the performance should be fine
            cs.sort_unstable();
            cs.reverse();
        }
    }

    fn filter(&mut self, rules: &FilterRules) -> () {
        for (_, cs) in self.0.iter_mut() {
            cs.retain(|c| !rules.is_matched(c.full_path()))
        }
    }

    pub fn reorgnize(&mut self, rules: &FilterRules) -> () {
        self.filter(rules);
        self.sort();
    }
}

impl Display for Candidates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alias_map = &config().tramp_aliases;
        let search_option = &config().search;
        for (prefix, cs) in self.0.iter() {
            let options = super::format::Options::new(prefix, alias_map);
            let paths: Vec<Vec<_>> = cs
                .iter()
                .take(search_option.limit)
                .map(Candidate::path_segs)
                .collect();
            Tree::new((&paths).into(), options).print(true, f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_last_ref(days: u64, sec: u64) -> LastRef {
        LastRef(now() - days * 24 * 60 * 60 - sec)
    }

    #[test]
    fn last_ref_ord_test() {
        let today = new_last_ref(0, 0);
        assert_eq!(today, new_last_ref(0, 100));

        let yest = new_last_ref(1, 0);
        assert_eq!(yest, new_last_ref(1, 100));

        assert!(today > yest);

        let yest_yest = new_last_ref(2, 0);
        let yest_yest_yest = new_last_ref(3, 0);
        assert_eq!(yest_yest, yest_yest_yest);
        assert!(yest_yest_yest > new_last_ref(4, 0));

        let last_month = new_last_ref(30, 0);
        assert!(yest_yest_yest > last_month);
        assert_eq!(last_month, new_last_ref(34, 0));
        assert!(last_month > new_last_ref(37, 0));
    }
}
