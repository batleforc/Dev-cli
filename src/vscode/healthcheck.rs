// Now we can open workspace in vscode through the open_code module.
// But the goal of the next part is to prevent the workspace from closing from the lack of activity.
// https://github.com/che-incubator/che-code/blob/6e0a908d58cacb380c216dde3af544d75e3913d5/build/scripts/entrypoint-volume.sh#L26
// https://github.com/kube-rs/kube/blob/main/examples/pod_portforward_hyper_http.rs#L39-L63
// https://github.com/che-incubator/che-code/blob/main/code/extensions/che-activity-tracker/src/activity-tracker-service.ts#L21
// https://github.com/eclipse/che/issues/22812
