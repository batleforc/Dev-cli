use tracing::event;

use crate::{
    config::CurrentWorkspace, devfile::lifecycle::find_pod_by_ws_name::find_pod_by_ws_name,
};

#[tracing::instrument(level = "trace")]
pub async fn get_workspace_container(current_workspace: CurrentWorkspace) {
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    let pod = match find_pod_by_ws_name(client, current_workspace.clone()).await {
        Some(pod) => pod,
        None => {
            event!(tracing::Level::ERROR, "Pod's not found");
            return;
        }
    };
    event!(
        tracing::Level::INFO,
        "Pod: {:?} => {:?} ",
        pod.metadata.name.unwrap(),
        pod.spec
            .unwrap()
            .containers
            .into_iter()
            .map(|c| c.name)
            .collect::<Vec<String>>()
    );
}
