use crate::crd::dev_work_space::DevWorkspace;
use kube::Api;
use tokio::time::{sleep, Duration};
use tracing::event;

pub async fn wait_for_status(
    devworkspace_api: Api<DevWorkspace>,
    ws_name: String,
    target_status: String,
    interval: u64,
    nb_retry: u64,
) -> Option<()> {
    let mut retry = 0;
    loop {
        retry += 1;
        let ws = match devworkspace_api.get(&ws_name).await {
            Ok(ws) => ws,
            Err(e) => {
                event!(tracing::Level::ERROR, "Could not get workspace: {}", e);
                return None;
            }
        };
        if let Some(status) = ws.status {
            if status.phase == Some(target_status.clone()) {
                return Some(());
            }
        }
        if retry >= nb_retry {
            event!(
                tracing::Level::ERROR,
                "Could not get workspace to status {}",
                target_status
            );
            return None;
        }
        sleep(Duration::from_millis(interval)).await;
    }
}
