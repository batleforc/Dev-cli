use crate::{
    config::CurrentWorkspace,
    crd::dev_work_space::DevWorkspace,
    devfile::lifecycle::{start_stop::start_stop_devworkspace, wait_for_status::wait_for_status},
};

#[tracing::instrument(level = "trace")]
pub async fn restart_workspace(current_workspace: CurrentWorkspace, wait: bool) {
    if current_workspace.is_in_pod() {
        tracing::error!(
            "You can't restart your current workspace from the inside of the workspaces"
        );
        return;
    }
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    if start_stop_devworkspace(client.clone(), current_workspace.clone(), false)
        .await
        .is_none()
    {
        tracing::error!("Could not restart workspace");
        return;
    } else {
        tracing::trace!("Workspace stopped");
    }
    let ws_name = current_workspace.workspace_name.clone().unwrap();
    let devworkspace_api = current_workspace.get_api::<DevWorkspace>(client.clone());
    if wait_for_status(
        devworkspace_api.clone(),
        ws_name.clone(),
        "Stopped".to_string(),
        2000,
        150, // Fail after 5 minutes
    )
    .await
    .is_none()
    {
        return;
    }
    if start_stop_devworkspace(client.clone(), current_workspace.clone(), true)
        .await
        .is_some()
    {
        tracing::info!("Workspace restarting");
    } else {
        return;
    }
    if wait
        && wait_for_status(
            devworkspace_api.clone(),
            ws_name.clone(),
            "Running".to_string(),
            2000,
            150, // Fail after 5 minutes
        )
        .await
        .is_some()
    {
        tracing::info!("Workspace restarted");
    }
}
