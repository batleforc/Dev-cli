use crate::{config::CurrentWorkspace, shell::find_pod_by_ws_name::find_pod_by_ws_name};
use k8s_openapi::api::core::v1::Pod;
use kube::api::AttachParams;
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
    let pod = match find_pod_by_ws_name(client.clone(), current_workspace.clone()).await {
        Some(pod) => pod,
        None => {
            event!(tracing::Level::ERROR, "Pod's not found");
            return;
        }
    };
    let container_cible = match select_pod(container_name, pod.clone()) {
        Some(c_name) => c_name,
        None => return,
    };
    let pod_api = current_workspace.get_api::<Pod>(client);

    let mut attach_param = AttachParams::interactive_tty();
    attach_param.container = Some(container_cible);

    let process = match pod_api
        .exec(&pod.metadata.name.unwrap(), vec![shell], &attach_param)
        .await
    {
        Ok(attached_process) => {
            event!(tracing::Level::TRACE, "Success createing remote process");
            attached_process
        }
        Err(err) => {
            event!(
                tracing::Level::ERROR,
                ?err,
                "Error while creating remote process"
            );
            return;
        }
    };
}

#[tracing::instrument(level = "trace")]
fn select_pod(target_name: Option<String>, pod: Pod) -> Option<String> {
    match target_name {
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
                return None;
            }
            Some(container_named)
        }
        None => match pod.spec.unwrap().containers.first() {
            Some(container) => {
                event!(
                    tracing::Level::INFO,
                    "Using first container : {}",
                    container.name.to_string()
                );
                Some(container.name.to_string())
            }
            None => {
                event!(
                    tracing::Level::ERROR,
                    "No container in the pod ? what did you do?"
                );
                return None;
            }
        },
    }
}
