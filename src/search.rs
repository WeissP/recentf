use crate::scored_paths::PathMap;

#[derive(Default, Debug)]
pub struct SearchArgs {
    tramp_ids: Option<Vec<usize>>,
    paths: Vec<String>,
    names: Vec<String>,
}

impl SearchArgs {
    pub fn new(tramp_ids: Option<Vec<usize>>, paths: Vec<String>, names: Vec<String>) -> Self {
        Self {
            tramp_ids,
            paths,
            names,
        }
    }

    pub fn from_prefix_str(args: Vec<String>) -> Self {
        let mut res = SearchArgs {
            tramp_ids: Some(vec![0, 1]),
            ..Default::default()
        };

        for arg in args {
            let mut chars = arg.chars();
            match chars.next() {
                Some('/') => res.paths.push(chars.as_str().to_owned()),
                Some('@') if arg.len() == 1 => res.tramp_ids = None,
                Some('@') => res.tramp_ids = Some(vec![str::parse::<usize>(&arg[1..]).unwrap()]),
                _ => res.names.push(arg.to_owned()),
            };
        }
        res
    }

    pub fn search(self) {
        let files = hisfile::search(self.tramp_ids, self.paths, self.names).unwrap();
        println!("{}", PathMap::from(&files));
    }
}
