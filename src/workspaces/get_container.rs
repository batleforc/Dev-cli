use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, client, Api};
use tracing::event;

use crate::config::CurrentWorkspace;

#[tracing::instrument(level = "trace")]
pub async fn get_workspace_container(current_workspace: CurrentWorkspace) {
    let client = client::Client::try_default().await.unwrap();
    let pods: Api<Pod> = match current_workspace.namespace {
        Some(namespace) => Api::namespaced(client, &namespace),
        None => Api::default_namespaced(client),
    };
    let mut lp = ListParams::default();
    if let Some(workspace_name) = current_workspace.workspace_name {
        let label = format!("controller.devfile.io/devworkspace_name={}", workspace_name);
        lp = lp.labels(&label);
    }
    for pod in pods.list(&lp).await.unwrap() {
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
}
