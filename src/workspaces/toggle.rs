use crate::{config::CurrentWorkspace, devfile::lifecycle::start_stop::start_stop_devworkspace};

#[tracing::instrument(level = "trace")]
pub async fn toggle_workspace(current_workspace: CurrentWorkspace, target_status: bool) {
    if current_workspace.is_in_pod() {
        tracing::warn!("You are editing your current workspace");
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
            true => {
                tracing::info!("Started : {:?} ", current_workspace.workspace_name.clone());
            }
            false => {
                tracing::info!("Stoped: {:?} ", current_workspace.workspace_name.clone());
            }
        };
    }
}
