use tokio::time::{sleep, Duration};
use tracing::event;

use crate::{
    config::CurrentWorkspace, crd::dev_work_space::DevWorkspace,
    devfile::lifecycle::start_stop::start_stop_devworkspace,
};

#[tracing::instrument(level = "trace")]
pub async fn restart_workspace(current_workspace: CurrentWorkspace) {
    if current_workspace.is_in_pod() {
        event!(
            tracing::Level::ERROR,
            "You can't restart your current workspace from the inside of the workspaces"
        );
        return;
    }
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    let devworkspace_api = current_workspace.get_api::<DevWorkspace>(client);
    if start_stop_devworkspace(devworkspace_api.clone(), current_workspace.clone(), false)
        .await
        .is_none()
    {
        event!(tracing::Level::ERROR, "Could not restart workspace");
        return;
    } else {
        event!(tracing::Level::TRACE, "Workspace stopped");
    }
    let ws_name = current_workspace.workspace_name.clone().unwrap();
    loop {
        let ws = match devworkspace_api.get(&ws_name).await {
            Ok(ws) => ws,
            Err(e) => {
                event!(tracing::Level::ERROR, "Could not get workspace: {}", e);
                return;
            }
        };
        if let Some(status) = ws.status {
            if status.phase == Some("Stopped".to_string()) {
                break;
            }
        }
        sleep(Duration::from_millis(2000)).await;
    }
    if start_stop_devworkspace(devworkspace_api.clone(), current_workspace.clone(), true)
        .await
        .is_some()
    {
        event!(tracing::Level::INFO, "Workspace restarting");
        return;
    }
}
