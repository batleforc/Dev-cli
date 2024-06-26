use kube::{
    api::{Patch, PatchParams},
    Client,
};
use serde_json::from_value;

use crate::{config::CurrentWorkspace, crd::dev_work_space::DevWorkspace};

#[tracing::instrument(level = "trace", skip(client))]
pub async fn start_stop_devworkspace(
    client: Client,
    current_workspace: CurrentWorkspace,
    running: bool,
) -> Option<DevWorkspace> {
    if let Some(ws_name) = current_workspace.workspace_name.clone() {
        let devworkspace_api = current_workspace.get_api::<DevWorkspace>(client);
        let js_patch = serde_json::json!([{
            "op": "replace",
            "path":"/spec/started",
            "value": running
        }]);
        let p_patch: json_patch::Patch = from_value(js_patch).unwrap();
        let params = PatchParams::apply("dev-cli");
        let patch = Patch::Json::<()>(p_patch);
        let res = devworkspace_api
            .patch(&ws_name.clone(), &params, &patch)
            .await;
        match res {
            Ok(ws) => {
                tracing::trace!(?ws, "ws updated");
                Some(ws)
            }
            Err(err) => {
                tracing::error!(?err, "Couldn't update ws");
                None
            }
        }
    } else {
        tracing::error!("No devworkspace selected");
        None
    }
}
