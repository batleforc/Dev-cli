use clap::Subcommand;

use super::get;

/// Handle the workspaces subcommand
#[derive(Subcommand)]
pub enum WorkSpaces {
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

impl WorkSpaces {
    /// Run the subcommand
    pub async fn run(&self) {
        match self {
            WorkSpaces::Get {} => get::get_current_workspace().await,
            WorkSpaces::GetContainer {} => todo!(),
            WorkSpaces::List {} => todo!(),
            WorkSpaces::Set {} => todo!(),
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
