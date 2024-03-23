use kube::{
    api::{Patch, PatchParams},
    Api,
};
use serde_json::from_value;
use tracing::event;

use crate::{config::CurrentWorkspace, crd::dev_work_space::DevWorkspace};

#[tracing::instrument(level = "trace")]
pub async fn start_stop_devworkspace(
    devworkspace_api: Api<DevWorkspace>,
    current_workspace: CurrentWorkspace,
    running: bool,
) -> Option<DevWorkspace> {
    if let Some(ws_name) = current_workspace.workspace_name {
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
                event!(tracing::Level::TRACE, ?ws, "ws updated");
                Some(ws)
            }
            Err(err) => {
                event!(tracing::Level::ERROR, ?err, "Couldn't update ws");
                None
            }
        }
    } else {
        event!(tracing::Level::ERROR, "Please select a devworkspace");
        None
    }
}
