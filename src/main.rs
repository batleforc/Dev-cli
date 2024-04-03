use clap::{Parser, Subcommand};
use dev_cli::{
    trace::{self, level::VerboseLevel},
    workspaces::command::WorkSpaces,
};
use tracing::{event, Level};

#[derive(Parser)]
#[command(name = "DevCli", about = "A simple cli to ease the dev process with EclipseChe/Devspaces", long_about = None, version, arg_required_else_help = true)]
struct Cli {
    /// Set log level
    #[arg(short, long, global = true, value_enum)]
    verbose: Option<VerboseLevel>,

    /// Enable trace logging, push trace to trace.log in a json format
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

        /// The namespace where your workspace is
        #[arg(short, long, global = true)]
        namespace: Option<String>,
        /// The name of the workspace
        #[arg(short, long, global = true)]
        workspace_name: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    println!(include_str!("dev-cli.art"));
    let cli = Cli::parse();
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
            namespace,
            workspace_name,
        }) => {
            if let Some(workspace) = workspace {
                workspace
                    .run(namespace.clone(), workspace_name.clone())
                    .await;
            } else {
            }
        }

        None => event!(Level::INFO, "No command provided"),
    }
}
