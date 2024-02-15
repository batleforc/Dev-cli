use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "DevCli", about = "A simple cli to ease the dev process with EclipseChe/Devspaces", long_about = None, version)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

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
    },
}
/// Handle the workspaces subcommand
#[derive(Subcommand)]
enum WorkSpaces {
    /// Get the current workspace (if you are in one, either the one selected)
    Get {},
    /// Get a list of all containers in the current workspace/pod
    GetContainer {},
    /// List all workspaces
    List {},
    /// Set the current workspace
    Set {},
    /// Start the current workspace
    Start {},
    /// Stop the current workspace
    Stop {},
    /// Restart the current workspace
    Restart {},
    /// Restart the current workspace from local devfile
    RestartLocal {},
    /// Spawn a shell in the selected container
    Shell {
        /// The name of the container to spawn the shell in
        name: String,

        /// The shell to spawn
        #[arg(short, long, default_value = "bash")]
        shell: String,
    },
    /// Open the selected workspace in vscode
    OpenVsCode {
        /// The name of the container to spawn the vscode in
        name: String,
    },
    /// Get the info to open the workspace in Idea
    InfoIdea {},
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Name: {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Config: {:?}", config_path);
    }

    match cli.debug {
        0 => println!("Debugging is off"),
        1 => println!("Debugging is kind of on"),
        2 => println!("Debugging is on"),
        _ => println!("Don't be crazy"),
    }

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
