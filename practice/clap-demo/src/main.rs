use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "git", long_about = None, about = "A fictional versioning CLI")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Clone {
        #[clap(value_parser)]
        remote: String,
    },
    #[clap(arg_required_else_help = true)]
    Push {
        #[clap(value_parser)]
        remote: String,
    },
    #[clap(arg_required_else_help = true)]
    Add {
        #[clap(required = true, value_parser)]
        path: Vec<PathBuf>,
    },
    Stash(Stash),
    #[clap(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
struct Stash {
    #[clap(subcommand)]
    command: Option<StashCommands>,

    #[clap(flatten)]
    push: StashPush,
}

#[derive(Debug, Subcommand)]
enum StashCommands {
    Push(StashPush),
    Pop {
        #[clap(value_parser)]
        stash: Option<String>,
    },
    Apply {
        #[clap(value_parser)]
        stash: Option<String>,
    },
}

#[derive(Debug, Args)]
struct StashPush {
    #[clap(short, long, value_parser)]
    message: Option<String>,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Clone { remote } => {
            println!("Cloning {}", remote);
        }
        Commands::Push { remote } => {
            println!("Pushing to {}", remote);
        }
        Commands::Add { path } => {
            println!("Adding {:?}", path);
        }
        Commands::Stash(stash) => {
            let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
            match stash_cmd {
                StashCommands::Push(push) => {
                    println!("Pushing {:?}", push);
                }
                StashCommands::Pop { stash } => {
                    println!("Popping {:?}", stash);
                }
                StashCommands::Apply { stash } => {
                    println!("Applying {:?}", stash);
                }
            }
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
}
