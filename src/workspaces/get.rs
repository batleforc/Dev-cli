use k8s_openapi::api::core::v1::Pod;
use tracing::event;

use crate::config::CurrentWorkspace;

#[tracing::instrument(level = "trace")]
pub async fn get_current_workspace(current_workspace: CurrentWorkspace) {
    event!(
        tracing::Level::INFO,
        "Current workspace: {:?} ",
        current_workspace
    );
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    let pods = current_workspace.get_api::<Pod>(client);
    for pod in pods.list(&Default::default()).await.unwrap() {
        event!(tracing::Level::INFO, "Pod: {:?}", pod.metadata.name);
    }
}
