use crate::{config::CurrentWorkspace, shell::find_pod_by_ws_name::find_pod_by_ws_name};
use tracing::event;

#[tracing::instrument(level = "trace")]
pub async fn spawn_shell(
    current_workspace: CurrentWorkspace,
    container_name: Option<String>,
    shell: String,
) {
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
    let container_cible = match container_name {
        Some(container_named) => {
            if !pod
                .spec
                .clone()
                .unwrap()
                .containers
                .into_iter()
                .any(|c| c.name == container_named)
            {
                event!(
                    tracing::Level::ERROR,
                    "Pod does not have container : {}",
                    container_named.to_string()
                );
                return;
            }
            container_named
        }
        None => match pod.spec.unwrap().containers.first() {
            Some(container) => {
                event!(
                    tracing::Level::INFO,
                    "Using first container : {}",
                    container.name.to_string()
                );
                container.name.to_string()
            }
            None => {
                event!(
                    tracing::Level::ERROR,
                    "No container in the pod ? what did you do?"
                );
                return;
            }
        },
    };
}
