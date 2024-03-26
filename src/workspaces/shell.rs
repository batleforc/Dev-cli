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

    let mut process = match pod_api
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

    let mut stdout_reader = match process.stdout() {
        Some(reader) => {
            event!(tracing::Level::TRACE, "Got stdout reader");
            reader
        }
        None => {
            event!(tracing::Level::ERROR, "Can't get stdout reader");
            return;
        }
    };

    let mut stdin_writer = match process.stdin() {
        Some(writer) => {
            event!(tracing::Level::TRACE, "Got stdin writer");
            writer
        }
        None => {
            event!(tracing::Level::ERROR, "Can't get stdin writer");
            return;
        }
    };

    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();

    tokio::spawn(async move {
        match tokio::io::copy(&mut stdin, &mut stdin_writer).await {
            Ok(is_ok) => {
                event!(tracing::Level::TRACE, "Output stdin code : {}", is_ok);
            }
            Err(err) => {
                event!(tracing::Level::ERROR, ?err, "Error while copying stdin");
            }
        };
    });
    // pipe stdout from ws to current stdout
    tokio::spawn(async move {
        match tokio::io::copy(&mut stdout_reader, &mut stdout).await {
            Ok(is_ok) => {
                event!(tracing::Level::TRACE, "Output stdout code : {}", is_ok);
            }
            Err(err) => {
                event!(tracing::Level::ERROR, ?err, "Error while copying stdout");
            }
        };
    });

    let status = process.take_status().unwrap().await;
    event!(tracing::Level::INFO, ?status, "End of session");
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
