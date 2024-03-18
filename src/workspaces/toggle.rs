use tracing::event;

use crate::{
    config::CurrentWorkspace, crd::dev_work_space::DevWorkspace,
    devfile::lifecycle::start_stop::start_stop_devworkspace,
};

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
    let devworkspace_api = current_workspace.get_api::<DevWorkspace>(client);
    if let Some(_) =
        start_stop_devworkspace(devworkspace_api, current_workspace.clone(), target_status).await
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
