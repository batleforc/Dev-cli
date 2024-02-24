use clap::{Parser, Subcommand};
use dev_cli::{
    trace::{self, level::VerboseLevel},
    workspaces::command::WorkSpaces,
};
use std::path::PathBuf;
use tracing::{event, Level};

#[derive(Parser)]
#[command(name = "DevCli", about = "A simple cli to ease the dev process with EclipseChe/Devspaces", long_about = None, version)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Set log level
    #[arg(short, long, global = true, value_enum)]
    verbose: Option<VerboseLevel>,

    #[arg(short, long, global = true)]
    trace: bool,

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
    println!(
        r" ____                   ____ _     ___ 
|  _ \  _____   __     / ___| |   |_ _|
| | | |/ _ \ \ / /____| |   | |    | | 
| |_| |  __/\ V /_____| |___| |___ | | 
|____/ \___| \_/       \____|_____|___|"
    );
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Name: {}", name);
    }

    let debug_level = match cli.verbose {
        Some(level) => {
            println!("Verbose is set to {:?}", level);
            level.into()
        }
        _ => Level::INFO,
    };
    trace::init::init_tracing(debug_level, cli.trace);

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                event!(Level::INFO, "Printing list");
            } else {
                event!(Level::INFO, "Not printing list");
            }
        }
        Some(Commands::Workspaces {
            workspace,
            config: _,
        }) => {
            if let Some(workspace) = workspace {
                workspace.run().await;
            }
        }

        None => event!(Level::INFO, "No command provided"),
    }
}
