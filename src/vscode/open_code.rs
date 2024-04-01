#[derive(Clone, Default)]
pub struct OpenCode {
    // is it needed ? Kubernetes context
    pub context: Option<String>,
    pub pod_name: Option<String>,
    pub namespace: Option<String>,
    pub container_name: Option<String>,
    pub container_image: Option<String>,
    pub path: Option<String>,
}

impl OpenCode {
    // https://github.com/vscode-kubernetes-tools/vscode-kubernetes-tools/issues/1207
    pub fn generate_path(&self) -> String {
        let mut base_uri = String::from("vscode-remote://k8s-container");
        if let Some(context) = &self.context {
            base_uri = format!("{}+context={}", base_uri, context);
        }
        if let Some(pod_name) = &self.pod_name {
            base_uri = format!("{}+podname={}", base_uri, pod_name);
        }
        if let Some(namespace) = &self.namespace {
            base_uri = format!("{}+namespace={}", base_uri, namespace);
        }
        if let Some(container_name) = &self.container_name {
            base_uri = format!("{}+name={}", base_uri, container_name);
        }
        if let Some(container_image) = &self.container_image {
            base_uri = format!("{}+image={}", base_uri, container_image);
        }
        if let Some(path) = &self.path {
            base_uri = format!("{}{}", base_uri, path);
        }
        base_uri
    }
}
