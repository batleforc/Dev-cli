use super::{get, get_container, list, open_vs_code, restart, shell, toggle};
use crate::config::CurrentWorkspace;
use clap::Subcommand;

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
    Restart {
        /// Wait for the workspace to be started
        #[arg(long)]
        wait: bool,
    },
    /// Restart the current workspace from local devfile
    RestartLocal {},
    /// Spawn a shell in the selected container
    Shell {
        /// The name of the container to spawn the shell in
        #[arg(long)]
        name: Option<String>,

        /// The shell to spawn
        shell: String,
    },
    /// Open the selected workspace in vscode
    OpenVsCode {
        /// The name of the container to spawn the vscode in
        #[arg(long)]
        name: Option<String>,

        /// Port for the healthcheck
        #[arg(long, default_value_t = 3333)]
        port: u16,

        /// Path in wich the workspace will be opened
        #[arg(long, short, default_value = "/projects/")]
        path: String,

        #[arg(long, short)]
        context: Option<String>,
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
                tracing::error!("Not in a workspace");
                return;
            }
        };
        if let Some(namespace) = namespace {
            current_workspace.namespace = Some(namespace.clone());
            tracing::trace!("Using namespace: {:?}", namespace);
        } else if let Some(namespace) = current_workspace.namespace.clone() {
            tracing::trace!("Using namespace: {:?}", namespace);
        } else {
            current_workspace.namespace = None;
            tracing::trace!("No namespace provided, using default namespace.");
        }
        if let Some(workspace_name) = workspace_name {
            current_workspace.workspace_name = Some(workspace_name.clone());
            tracing::trace!("Using workspace: {:?}", workspace_name)
        } else if let Some(workspace_name) = current_workspace.workspace_name.clone() {
            tracing::trace!("Using workspace: {:?}", workspace_name);
        } else {
            current_workspace.workspace_name = None;
            tracing::trace!("No workspace_name provided.");
        }
        match self {
            WorkSpaces::Get {} => get::get_current_workspace(current_workspace).await,
            WorkSpaces::GetContainer {} => {
                get_container::get_workspace_container(current_workspace).await
            }
            WorkSpaces::List {} => list::list_workspace(current_workspace).await,
            WorkSpaces::Start {} => toggle::toggle_workspace(current_workspace, true).await,
            WorkSpaces::Stop {} => toggle::toggle_workspace(current_workspace, false).await,
            WorkSpaces::Restart { wait } => {
                restart::restart_workspace(current_workspace, *wait).await
            }
            WorkSpaces::RestartLocal {} => todo!(),
            WorkSpaces::Shell {
                name,
                shell: shell_content,
            } => {
                if let Err(err) =
                    shell::spawn_shell(current_workspace, name.clone(), shell_content.to_string())
                        .await
                {
                    tracing::error!(?err, "Unhandled error from crossterm or other");
                }
            }
            WorkSpaces::OpenVsCode {
                name,
                port,
                path,
                context,
            } => {
                open_vs_code::open_vs_code(
                    current_workspace,
                    name.clone(),
                    *port,
                    path.to_string(),
                    context.clone(),
                )
                .await
            }
            WorkSpaces::InfoIdea {} => todo!(),
        }
    }
}
