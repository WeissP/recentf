use clap::{Args, Parser, Subcommand};
use hisfile;

use search::SearchArgs;
use upsert::Upsert;

mod scored_paths;
mod search;
mod tree;
mod upsert;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[clap(value_parser, required = true)]
        emacs_path: String,
    },
    Search(Search),
    PrefixedSearch {
        #[clap(value_parser, required = true)]
        args: Vec<String>,
    },
    Test,
    Clean,
}

#[derive(Args, Debug)]
struct Search {
    // None: tramp_id=0
    // Some(0): all
    // Some(id): tramp_id<=id
    #[clap(value_parser, short, long, value_name = "TRAMP_ID")]
    tramp_range: Option<usize>,
    #[clap(value_parser, short, long, value_name = "PATH")]
    path: Option<Vec<String>>,
    #[clap(value_parser, short, long, value_name = "NAME")]
    name: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { emacs_path: x } => Upsert::from(x.as_str()).insert().unwrap(),
        Commands::Search(x) => {
            let ids = match x.tramp_range {
                None => Some(vec![0]),
                Some(0) => None,
                Some(id) => Some((0..=id).collect()),
            };
            SearchArgs::new(ids, x.path.unwrap_or_default(), x.name.unwrap_or_default()).search();
        }
        Commands::Test => hisfile::test(),
        Commands::Clean => {
            hisfile::check_ban_rules().unwrap();
            hisfile::check_existence().unwrap();
        }
        Commands::PrefixedSearch { args } => SearchArgs::from_prefix_str(args).search(),
    }
}
