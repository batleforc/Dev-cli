use kube::Api;
use tokio::time::{sleep, Duration};
use tracing::event;

use crate::{
    config::CurrentWorkspace, crd::dev_work_space::DevWorkspace,
    devfile::lifecycle::start_stop::start_stop_devworkspace,
};

#[tracing::instrument(level = "trace")]
pub async fn open_vs_code(
    current_workspace: CurrentWorkspace,
    container_name: Option<String>,
    port: u16,
    path: String,
    context: Option<String>,
) {
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    let devworkspace_api = current_workspace.get_api::<DevWorkspace>(client);
}
