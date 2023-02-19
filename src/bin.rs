use clap::{arg, command, Args, Parser, Subcommand};
use recentf_lib::{clean, config, database, search::Query, tramp};
use std::str::FromStr;

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
    Remove {
        #[clap(value_parser, required = true)]
        #[arg(help("the path in the form of emacs to be removed"))]
        emacs_path: String,
    },
    Search {
        #[clap(value_parser, required = true)]
        query: String,
    },
    Test,
    Clean,
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
struct AddArgs {
    #[arg(help("the path in the form of emacs to be added"))]
    emacs_path: String,
    #[arg(
        long,
        short,
        help("A flag that indicates whether the added path should be favourite")
    )]
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
            if arg.favourite {
                database::change_status(
                    &mut conn,
                    tramp_prefix,
                    file_path,
                    recentf_lib::search::Status::Favourite,
                )
                .await
                .unwrap()
            }
        }
        Commands::Remove { emacs_path: p } => {
            let (tramp_prefix, file_path) = tramp::split(&p);
            database::change_deleted_flag(&mut conn, tramp_prefix, file_path, true)
                .await
                .unwrap();
        }
        Commands::Search { query: x } => {
            let query = Query::from_str(&x).unwrap();
            let mut res = database::search(&mut conn, query).await.unwrap();
            res.reorgnize(&config::config().filter);
            println!("{}", res);
        }
        Commands::Test => println!("test"),
        Commands::Clean => {
            clean::clean().unwrap();
        }
    }
}
