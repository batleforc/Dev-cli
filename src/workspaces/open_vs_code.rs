use crate::shell::select_pod::select_pod;
use crate::vscode::healthcheck;
use crate::{
    devfile::lifecycle::ask_if_pod_should_up::ask_if_pod_should_up,
    workspaces::open_vs_code::wait_for_status::wait_for_status,
};

use crate::{
    config::CurrentWorkspace,
    crd::dev_work_space::DevWorkspace,
    devfile::lifecycle::{
        find_pod_by_ws_name::find_pod_by_ws_name, start_stop::start_stop_devworkspace,
        wait_for_status,
    },
};

#[tracing::instrument(level = "trace")]
pub async fn open_vs_code(
    current_workspace: CurrentWorkspace,
    mut container_name: Option<String>,
    port: u16,
    path: String,
    context: Option<String>,
) {
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    // Check if the workspace is already running
    // if not ask the user if they want to start it
    // if yes start it
    // if no return
    let pod = match find_pod_by_ws_name(client.clone(), current_workspace.clone()).await {
        Some(pod) => pod,
        None => {
            if ask_if_pod_should_up().await {
                start_stop_devworkspace(client.clone(), current_workspace.clone(), true).await;
                if wait_for_status(
                    current_workspace.get_api::<DevWorkspace>(client.clone()),
                    current_workspace.workspace_name.clone().unwrap(),
                    "Running".to_string(),
                    2000,
                    150, // Fail after 5 minutes
                )
                .await
                .is_none()
                {
                    return;
                }
                find_pod_by_ws_name(client.clone(), current_workspace.clone())
                    .await
                    .unwrap()
            } else {
                return;
            }
        }
    };
    container_name = select_pod(container_name, pod.clone());

    let open_code = crate::vscode::open_code::OpenCode {
        context,
        pod_name: Some(pod.metadata.name.clone().unwrap()),
        namespace: Some(pod.metadata.namespace.clone().unwrap()),
        container_name: container_name.clone(),
        container_image: Some(
            pod.spec.clone().unwrap().containers[0]
                .image
                .clone()
                .unwrap(),
        ),
        path: Some(path),
    };
    open_code.open();
    healthcheck::healthcheck(client, current_workspace, pod.metadata.name.unwrap(), port).await;
}
