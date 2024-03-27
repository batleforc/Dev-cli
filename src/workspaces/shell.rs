use crate::{
    config::CurrentWorkspace,
    shell::{find_pod_by_ws_name::find_pod_by_ws_name, select_pod, start_it_shell::start_it_shell},
};
use k8s_openapi::api::core::v1::Pod;
use tracing::event;

#[tracing::instrument(level = "trace")]
pub async fn spawn_shell(
    current_workspace: CurrentWorkspace,
    container_name: Option<String>,
    shell: String,
) -> anyhow::Result<()> {
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return Ok(()),
    };
    let pod = match find_pod_by_ws_name(client.clone(), current_workspace.clone()).await {
        Some(pod) => pod,
        None => {
            event!(tracing::Level::ERROR, "Pod's not found");
            return Ok(());
        }
    };
    let container_cible = match select_pod::select_pod(container_name, pod.clone()) {
        Some(c_name) => c_name,
        None => return Ok(()),
    };
    let pod_api = current_workspace.get_api::<Pod>(client);

    start_it_shell(pod.metadata.name.unwrap(), container_cible, shell, pod_api).await?;

    Ok(())
}
