macro_rules! query {
    ($e:expr) => {
        $e.iter()
            .cloned()
            .map(|x: &'static str| x.to_string())
            .collect()
    };

    ($aliases:expr, $paths:expr, $names:expr) => {
        Query::new(
            crate::common::search::query!($aliases),
            crate::common::search::query!($paths),
            crate::common::search::query!($names),
        )
    };
}
pub(crate) use query;

macro_rules! search {
    ($pool:expr, $aliases:expr, $paths:expr, $names:expr) => {
        database::search(
            $pool,
            crate::common::search::query!($aliases, $paths, $names),
        )
        .await
        .unwrap()
    };
}
pub(crate) use search;

use recentf_lib::search::Candidates;

pub fn paths_with_id(can: Candidates, id: i16) -> Vec<String> {
    can.get(id)
        .expect(&format!(
            "candidates has no value on id: {}. Candidates:\n{:?}",
            id, can
        ))
        .iter()
        .map(|x| x.full_path().to_string())
        .collect()
}