use dev_cli::{config::CurrentWorkspace, trace};
use tracing::{event, Level};

extern crate dev_cli;

#[tokio::main]
pub async fn main() {
    trace::init::init_tracing(Level::INFO, false);
    let mut current_workspace = match CurrentWorkspace::try_from_env() {
        Some(workspace) => workspace,
        None => {
            event!(tracing::Level::ERROR, "Not in a workspace");
            return;
        }
    };
    current_workspace.workspace_name = Some("dev-cli".to_string());
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    // check if workspace is up by getting the pod
    let pod = match dev_cli::devfile::lifecycle::find_pod_by_ws_name::find_pod_by_ws_name(
        client.clone(),
        current_workspace.clone(),
    )
    .await
    {
        Some(pod) => pod,
        None => {
            event!(
                tracing::Level::ERROR,
                "No pod found for workspace, should i start the workspace?"
            );
            return;
        }
    };
    // Create open code object
    let open_code = dev_cli::vscode::open_code::OpenCode {
        context: None,
        pod_name: Some(pod.metadata.name.clone().unwrap()),
        namespace: Some(pod.metadata.namespace.clone().unwrap()),
        container_name: Some(pod.spec.clone().unwrap().containers[0].name.clone()),
        container_image: Some(
            pod.spec.clone().unwrap().containers[0]
                .image
                .clone()
                .unwrap(),
        ),
        path: Some("/projects/".to_string()),
    };
    // Open the workspace in vscode
    open_code.open();
}
