use clap::Subcommand;
use tracing::event;

use crate::config::CurrentWorkspace;

use super::{get, get_container};

/// Handle the workspaces subcommand
#[derive(Subcommand)]
pub enum WorkSpaces {
    /// Get the current workspace (if you are in one, either the one selected)
    Get {},
    /// Get a list of all containers in the current workspace/pod
    GetContainer {},
    /// List all workspaces
    List {},
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

impl WorkSpaces {
    /// Run the subcommand
    pub async fn run(&self, namespace: Option<String>, workspace_name: Option<String>) {
        let mut current_workspace = match CurrentWorkspace::try_from_env() {
            Some(workspace) => workspace,
            None => {
                event!(tracing::Level::ERROR, "Not in a workspace");
                return;
            }
        };
        if let Some(namespace) = namespace {
            current_workspace.namespace = Some(namespace.clone());
            event!(tracing::Level::TRACE, "Using namespace: {:?}", namespace);
        } else if let Some(namespace) = current_workspace.namespace.clone() {
            event!(tracing::Level::TRACE, "Using namespace: {:?}", namespace);
        } else {
            current_workspace.namespace = None;
            event!(
                tracing::Level::TRACE,
                "No namespace provided, using default namespace."
            );
        }
        if let Some(workspace_name) = workspace_name {
            current_workspace.workspace_name = Some(workspace_name.clone());
            event!(
                tracing::Level::TRACE,
                "Using workspace: {:?}",
                workspace_name
            );
        } else if let Some(workspace_name) = current_workspace.workspace_name.clone() {
            event!(
                tracing::Level::TRACE,
                "Using workspace: {:?}",
                workspace_name
            );
        } else {
            current_workspace.workspace_name = None;
            event!(tracing::Level::TRACE, "Using no workspace_name.");
        }
        match self {
            WorkSpaces::Get {} => get::get_current_workspace().await,
            WorkSpaces::GetContainer {} => {
                get_container::get_workspace_container(current_workspace).await
            }
            WorkSpaces::List {} => todo!(),
            WorkSpaces::Start {} => todo!(),
            WorkSpaces::Stop {} => todo!(),
            WorkSpaces::Restart {} => todo!(),
            WorkSpaces::RestartLocal {} => todo!(),
            WorkSpaces::Shell { name: _, shell: _ } => todo!(),
            WorkSpaces::OpenVsCode { name: _ } => todo!(),
            WorkSpaces::InfoIdea {} => todo!(),
        }
    }
}
