pub mod clean;
pub mod config;
pub mod database;
pub mod search;
pub mod tramp;
use clap::{arg, command, Args, Parser, Subcommand};
use search::Query;
use std::{collections::HashMap, path::PathBuf, str::FromStr};
// use hisfile;
use tramp::TrampPath;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    Search {
        #[clap(value_parser, required = true)]
        query: String,
    },
    Test,
    Clean,
}

#[derive(Args)]
struct AddArgs {
    emacs_path: String,
    #[arg(default_value_t = false)]
    favourite: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut conn = database::connect(config::database_path()).await.unwrap();
    match cli.command {
        Commands::Add(arg) => {
            let (tramp_prefix, file_path) = tramp::split(&arg.emacs_path);
            database::upsert(&mut conn, tramp_prefix, file_path)
                .await
                .unwrap();
        }
        Commands::Search { query: x } => {
            let query = Query::from_str(&x).unwrap();
            let res = database::search(&mut conn, query).await.unwrap();
            // tramp::pretty_print(res);
        }
        Commands::Test => println!("test"),
        Commands::Clean => {
            clean::clean().unwrap();
        }
    }
}
