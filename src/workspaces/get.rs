use tracing::event;

use crate::config::CurrentWorkspace;

#[tracing::instrument]
pub async fn get_current_workspace() {
    let current_workspace = match CurrentWorkspace::try_from_env() {
        Some(workspace) => workspace,
        None => {
            event!(tracing::Level::ERROR, "Not in a workspace");
            return;
        }
    };
    event!(
        tracing::Level::INFO,
        "Current workspace: {:?} ",
        current_workspace
    )
}
