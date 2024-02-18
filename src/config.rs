use std::fmt::Debug;

pub struct CurrentWorkspace {
    pub namespace: String,
    pub workspace_name: String,
    pub id: String,
    pub podname: String,
    pub is_in_pod: bool,
}

impl CurrentWorkspace {
    pub fn new(
        namespace: String,
        id: String,
        workspace_name: String,
        podname: String,
        is_in_pod: bool,
    ) -> Self {
        Self {
            namespace,
            id,
            workspace_name,
            podname,
            is_in_pod,
        }
    }

    pub fn try_from_env() -> Option<Self> {
        let namespace = std::env::var("DEVWORKSPACE_NAMESPACE").ok()?;
        let id = std::env::var("DEVWORKSPACE_ID").ok()?;
        let workspace_name = std::env::var("DEVWORKSPACE_NAME").ok()?;
        let podname = std::env::var("HOSTNAME").ok()?;
        let is_in_workspace = true;
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
            .field("id", &self.id)
            .field("workspace_name", &self.workspace_name)
            .field("podname", &self.podname)
            .field("is_in_pod", &self.is_in_pod)
            .finish()
    }
}
