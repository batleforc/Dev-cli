use k8s_openapi::api::core::v1::Pod;
use kube::{client, Api};
use tracing::event;

use crate::config::CurrentWorkspace;

#[tracing::instrument(level = "trace")]
pub async fn get_current_workspace(current_workspace: CurrentWorkspace) {
    event!(
        tracing::Level::INFO,
        "Current workspace: {:?} ",
        current_workspace
    );
    let client = client::Client::try_default().await.unwrap();

    let pods: Api<Pod> = Api::default_namespaced(client);
    for pod in pods.list(&Default::default()).await.unwrap() {
        event!(tracing::Level::INFO, "Pod: {:?}", pod.metadata.name);
    }
}
