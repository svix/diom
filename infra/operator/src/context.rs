use std::sync::Arc;

use diom::{DiomClient, DiomOptions};
use k8s_openapi::api::{apps::v1::StatefulSet, core::v1::Service, policy::v1::PodDisruptionBudget};
use kube::{Api, Client, ResourceExt, api::PatchParams};

use crate::{
    crd::DiomCluster,
    error::{Error, Result},
    resources::services,
};

pub(crate) const FIELD_MANAGER: &str = "diom-operator";

pub fn admin_token() -> Option<String> {
    std::env::var("DIOM_ADMIN_TOKEN").ok()
}

pub(crate) struct ClusterCtx {
    pub cluster: Arc<DiomCluster>,
    pub client: Client,
    pub ns: String,
    pub name: String,
    pub diom_client: Option<Arc<DiomClient>>,
}

impl ClusterCtx {
    pub(crate) fn new(cluster: Arc<DiomCluster>, client: Client) -> Result<Self> {
        let ns = cluster
            .namespace()
            .ok_or(Error::MissingField("namespace"))?;
        let name = cluster.name_any();

        let diom_client = admin_token().map(|token| {
            Arc::new(DiomClient::new(
                token,
                Some(DiomOptions {
                    debug: false,
                    server_url: Some(format!(
                        "http://{}.{}.svc.cluster.local:{}",
                        services::lb_svc_name(&name),
                        ns,
                        cluster.spec.diom.api_port,
                    )),
                    ..Default::default()
                }),
            ))
        });
        Ok(Self {
            cluster,
            client,
            ns,
            name,
            diom_client,
        })
    }

    pub(crate) fn cluster_api(&self) -> Api<DiomCluster> {
        Api::namespaced(self.client.clone(), &self.ns)
    }

    pub(crate) fn sts_api(&self) -> Api<StatefulSet> {
        Api::namespaced(self.client.clone(), &self.ns)
    }

    pub(crate) fn pdb_api(&self) -> Api<PodDisruptionBudget> {
        Api::namespaced(self.client.clone(), &self.ns)
    }

    pub(crate) fn svc_api(&self) -> Api<Service> {
        Api::namespaced(self.client.clone(), &self.ns)
    }

    pub(crate) fn pp(&self) -> PatchParams {
        PatchParams::apply(FIELD_MANAGER).force()
    }

    pub(crate) fn status_pp(&self) -> PatchParams {
        PatchParams::apply(FIELD_MANAGER)
    }
}
