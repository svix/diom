use std::sync::Arc;

use k8s_openapi::api::{apps::v1::StatefulSet, core::v1::Service, policy::v1::PodDisruptionBudget};
use kube::{Api, Client, ResourceExt, api::PatchParams};

use crate::{
    crd::DiomCluster,
    error::{Error, Result},
};

pub(crate) const FIELD_MANAGER: &str = "diom-operator";

pub(crate) struct ClusterCtx {
    pub cluster: Arc<DiomCluster>,
    pub client: Client,
    pub ns: String,
    pub name: String,
}

impl ClusterCtx {
    pub(crate) fn new(cluster: Arc<DiomCluster>, client: Client) -> Result<Self> {
        let ns = cluster
            .namespace()
            .ok_or(Error::MissingField("namespace"))?;
        let name = cluster.name_any();
        Ok(Self {
            cluster,
            client,
            ns,
            name,
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
