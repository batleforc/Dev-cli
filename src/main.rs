use clap::{Parser, Subcommand};
use dev_cli::{trace::level::VerboseLevel, workspaces::command::WorkSpaces};
use std::path::PathBuf;
use tracing::Level;

#[derive(Parser)]
#[command(name = "DevCli", about = "A simple cli to ease the dev process with EclipseChe/Devspaces", long_about = None, version)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Set log level
    #[arg(short, long, global = true, value_enum)]
    verbose: Option<VerboseLevel>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
    Workspaces {
        #[command(subcommand)]
        workspace: Option<WorkSpaces>,

        /// Optional config file
        #[arg(short, long, global = true, value_name = "FILE")]
        config: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Name: {}", name);
    }

    let debug_level = match cli.verbose {
        Some(level) => {
            println!("Verbose is set to {:?}", level);
            level.into()
        }
        _ => Level::ERROR,
    };

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing list");
            } else {
                println!("Not printing list");
            }
        }
        &Some(Commands::Workspaces { .. }) => todo!(),

        None => println!("No subcommand"),
    }
}
