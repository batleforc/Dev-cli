use dev_cli::{config::CurrentWorkspace, crd::dev_work_space::DevWorkspace, trace};
use kube::api::{Patch, PatchParams};
use serde_json::from_value;
use tracing::{event, Level};

extern crate dev_cli;

#[tokio::main]
pub async fn main() {
    trace::init::init_tracing(Level::INFO, false);
    // let mut dev_filefs = match File::open("./devfile.yaml") {
    //     Ok(file) => file,
    //     Err(err) => {
    //         event!(tracing::Level::ERROR, "Could not open file: {:?}", err);
    //         return;
    //     }
    // };
    // let mut contents = String::new();
    // match dev_filefs.read_to_string(&mut contents) {
    //     Ok(content) => content,
    //     Err(err) => {
    //         event!(tracing::Level::ERROR, "Could not read file: {:?}", err);
    //         return;
    //     }
    // };
    // let version = DevFileVersion::validate(contents.clone());
    // println!("{:?}", version);
    let mut current_workspace = match CurrentWorkspace::try_from_env() {
        Some(workspace) => workspace,
        None => {
            event!(tracing::Level::ERROR, "Not in a workspace");
            return;
        }
    };
    current_workspace.workspace_name = Some("build-docker".to_string());
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    let devworkspace_api = current_workspace.get_api::<DevWorkspace>(client);

    if let Some(ws_name) = current_workspace.workspace_name {
        let js_patch = serde_json::json!([{
            "op": "replace",
            "path":"/spec/started",
            "value": false
        }]);
        let p_patch: json_patch::Patch = from_value(js_patch).unwrap();
        let params = PatchParams::apply("dev-cli");
        let patch = Patch::Json::<()>(p_patch);
        let res = devworkspace_api
            .patch(&ws_name.clone(), &params, &patch)
            .await;
        event!(tracing::Level::INFO, "{:?}", res);
    }
}
