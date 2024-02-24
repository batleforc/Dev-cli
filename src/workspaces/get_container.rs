use k8s_openapi::api::core::v1::Pod;
use kube::{client, Api};
use tracing::event;

use crate::config::CurrentWorkspace;

#[tracing::instrument]
pub async fn get_workspace_container() {
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
    );
    let client = client::Client::try_default().await.unwrap();

    let pods: Api<Pod> = Api::default_namespaced(client);
    for pod in pods.list(&Default::default()).await.unwrap() {
        event!(tracing::Level::INFO, "Pod: {:?}", pod.metadata.name);
        event!(
            tracing::Level::INFO,
            "Pod: {:?}",
            pod.spec
                .unwrap()
                .containers
                .into_iter()
                .map(|c| c.name)
                .collect::<Vec<String>>()
        );
    }
}
