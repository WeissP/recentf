use hisfile::cfg;
use std::{fmt::Display, path::Path};

use hisfile::File;

use crate::tree::Tree;

pub struct ScoredPath<'a> {
    pub fullpath: Vec<&'a str>,
    score: f64,
}

#[derive(Default)]
pub struct ScoredPaths<'a> {
    pub paths: Vec<ScoredPath<'a>>,
    pub tramp_id: usize,
}

impl<'a> ScoredPaths<'a> {
    pub fn sort(&mut self) {
        self.paths
            .sort_by(|x, y| x.score.partial_cmp(&y.score).unwrap())
    }
}

#[derive(Default)]
pub struct PathMap<'a>(Vec<ScoredPaths<'a>>);

impl<'a> Display for PathMap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|x| Tree::try_from(x).unwrap())
            .try_for_each(|x| x.0.print(true, f))
    }
}

impl<'a> From<&'a Vec<File>> for PathMap<'a> {
    fn from(files: &'a Vec<File>) -> Self {
        let mut op_map = Vec::<Option<ScoredPaths>>::new();
        for f in files {
            let id = f.tramp_id.0;
            let new = || ScoredPath {
                fullpath: Path::new(&f.fullpath[1..]) // remove the root slash
                    .iter()
                    .map(|x| x.to_str().unwrap())
                    .collect(),
                score: f.score(),
            };

            let new_sp = || {
                Some(ScoredPaths {
                    paths: vec![new()],
                    tramp_id: id,
                })
            };
            match op_map.get_mut(id) {
                Some(Some(sps)) => sps.paths.push(new()),
                Some(None) => op_map[id] = new_sp(),
                None => {
                    op_map.resize_with(id + 1, Default::default);
                    op_map[id] = new_sp();
                }
            };
        }
        PathMap(
            op_map
                .into_iter()
                .rev()
                .flatten() // remove None
                .map(|mut x| {
                    x.sort();
                    x.paths.truncate(cfg.limit);
                    x
                })
                .collect(),
        )
    }
}
