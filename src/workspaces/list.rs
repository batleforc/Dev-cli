use kube::{client, Api};
use tracing::event;

use crate::{config::CurrentWorkspace, crd::dev_work_space::DevWorkspace};

#[tracing::instrument(level = "trace")]
pub async fn list_workspace(current_workspace: CurrentWorkspace) {
    let client = client::Client::try_default().await.unwrap();
    let devworkspace_api: Api<DevWorkspace> = match current_workspace.namespace {
        Some(namespace) => Api::namespaced(client, &namespace),
        None => Api::default_namespaced(client),
    };
    event!(tracing::Level::INFO, "DevWorkSpace :");
    for devworkspace in devworkspace_api.list(&Default::default()).await.unwrap() {
        if current_workspace.is_in_pod
            && current_workspace.workspace_id.clone().is_some()
            && current_workspace.workspace_id.clone().unwrap()
                == devworkspace.status.clone().unwrap().devworkspace_id
        {
            event!(
                tracing::Level::INFO,
                "Ns : {} => {} , Status : {} and Current",
                devworkspace.metadata.namespace.unwrap(),
                devworkspace.metadata.name.unwrap(),
                devworkspace
                    .status
                    .unwrap()
                    .phase
                    .unwrap_or("Unknown".to_string()),
            );
        } else {
            event!(
                tracing::Level::INFO,
                "Ns : {} => {} {}",
                devworkspace.metadata.namespace.unwrap(),
                devworkspace.metadata.name.unwrap(),
                devworkspace
                    .status
                    .unwrap()
                    .phase
                    .unwrap_or("Unknown".to_string()),
            );
        }
    }
}
