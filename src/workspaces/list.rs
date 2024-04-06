use crate::{config::CurrentWorkspace, crd::dev_work_space::DevWorkspace};

#[tracing::instrument(level = "trace")]
pub async fn list_workspace(current_workspace: CurrentWorkspace) {
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    let devworkspace_api = current_workspace.get_api::<DevWorkspace>(client);
    tracing::info!("DevWorkSpace :");
    for devworkspace in devworkspace_api.list(&Default::default()).await.unwrap() {
        if current_workspace.is_in_pod()
            && current_workspace.workspace_id.clone().is_some()
            && current_workspace.workspace_id.clone().unwrap()
                == devworkspace.status.clone().unwrap().devworkspace_id
        {
            tracing::info!(
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
            tracing::info!(
                "Ns : {} => {} , Status : {}",
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
