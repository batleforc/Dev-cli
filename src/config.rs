use std::fmt::Debug;

pub struct CurrentWorkspace {
    pub namespace: Option<String>,
    pub workspace_name: Option<String>,
    pub workspace_id: Option<String>,
    pub podname: Option<String>,
    pub is_in_pod: bool,
}

impl CurrentWorkspace {
    pub fn new(
        namespace: Option<String>,
        id: Option<String>,
        workspace_name: Option<String>,
        podname: Option<String>,
        is_in_pod: bool,
    ) -> Self {
        Self {
            namespace,
            workspace_id: id,
            workspace_name,
            podname,
            is_in_pod,
        }
    }

    pub fn try_from_env() -> Option<Self> {
        let namespace = std::env::var("DEVWORKSPACE_NAMESPACE").ok();
        let id = std::env::var("DEVWORKSPACE_ID").ok();
        let workspace_name = std::env::var("DEVWORKSPACE_NAME").ok();
        let podname = std::env::var("HOSTNAME").ok();
        let mut is_in_workspace = true;
        if namespace.is_none() && id.is_none() && workspace_name.is_none() {
            is_in_workspace = false;
        }
        Some(Self::new(
            namespace,
            id,
            workspace_name,
            podname,
            is_in_workspace,
        ))
    }
}

impl Debug for CurrentWorkspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CurrentWorkspace")
            .field("namespace", &self.namespace)
            .field("id", &self.workspace_id)
            .field("workspace_name", &self.workspace_name)
            .field("podname", &self.podname)
            .field("is_in_pod", &self.is_in_pod)
            .finish()
    }
}
