use std::fmt::Debug;

use k8s_openapi::NamespaceResourceScope;
use kube::{client, Api, Client, Resource};
use tracing::event;

#[derive(Clone)]
pub struct CurrentWorkspace {
    pub namespace: Option<String>,
    pub workspace_name: Option<String>,
    pub workspace_id: Option<String>,
    pub podname: Option<String>,
    is_in_pod: bool,
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
    pub fn is_in_pod(&self) -> bool {
        let current_workspace_name = std::env::var("DEVWORKSPACE_NAME").ok();
        return self.is_in_pod && current_workspace_name == self.workspace_name;
    }

    #[tracing::instrument(level = "trace")]
    pub async fn get_client(&self) -> Option<Client> {
        match client::Client::try_default().await {
            Ok(iencli) => {
                event!(tracing::Level::TRACE, "Kube client created");
                Some(iencli)
            }
            Err(err) => {
                event!(
                    tracing::Level::ERROR,
                    "Could not instanciate kube Client : {:?}",
                    err
                );
                None
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(client))]
    pub fn get_api<T: Resource<Scope = NamespaceResourceScope>>(&self, client: Client) -> Api<T>
    where
        <T as kube::Resource>::DynamicType: Default,
    {
        match &self.namespace {
            Some(namespace) => Api::namespaced(client, &namespace),
            None => Api::default_namespaced(client),
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
