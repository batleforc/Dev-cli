use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Client};

use crate::config::CurrentWorkspace;

#[tracing::instrument(level = "trace", skip(client))]
pub async fn find_pod_by_ws_name(
    client: Client,
    current_workspace: CurrentWorkspace,
) -> Option<Pod> {
    let pods = current_workspace.get_api::<Pod>(client);
    let mut lp = ListParams::default();
    if let Some(workspace_name) = current_workspace.workspace_name {
        let label = format!("controller.devfile.io/devworkspace_name={}", workspace_name);
        lp = lp.labels(&label);
    }
    let list_pod = match pods.list(&lp).await {
        Ok(list) => {
            tracing::trace!(?list, "Got List of pod from kube");
            list
        }
        Err(err) => {
            tracing::error!(?err, "Couldn't get pods");
            return None;
        }
    };

    match list_pod.into_iter().next() {
        Some(pod) => {
            tracing::trace!(?pod, "Pod found");
            Some(pod)
        }
        None => {
            tracing::error!("No pods in list");
            None
        }
    }
}
