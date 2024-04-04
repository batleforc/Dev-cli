use tracing::event;

use crate::{config::CurrentWorkspace, devfile::lifecycle::start_stop::start_stop_devworkspace};

#[tracing::instrument(level = "trace")]
pub async fn toggle_workspace(current_workspace: CurrentWorkspace, target_status: bool) {
    if current_workspace.is_in_pod() {
        event!(
            tracing::Level::WARN,
            "You are editing your current workspace"
        );
    }
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    if start_stop_devworkspace(client, current_workspace.clone(), target_status)
        .await
        .is_some()
    {
        match target_status {
            true => event!(
                tracing::Level::INFO,
                "Started : {:?} ",
                current_workspace.workspace_name.clone()
            ),
            false => event!(
                tracing::Level::INFO,
                "Stoped: {:?} ",
                current_workspace.workspace_name.clone()
            ),
        };
    }
}
