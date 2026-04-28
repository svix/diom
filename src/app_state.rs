use std::sync::Arc;

use diom_authorization::{AccessRuleList, Permissions};
use diom_core::Monotime;
use diom_msgs::TopicPublishNotifier;
use diom_proto::{InternalClient, InternalRequestError};
use fjall_utils::{Databases, ReadonlyDatabases};
use opentelemetry::metrics::Meter;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    cfg::Configuration,
    core::{auth::FifoCache, jwt::JwtVerifier},
};

#[derive(Clone)]
pub struct AppState {
    pub(crate) cfg: Configuration,

    pub(crate) namespace_state: diom_namespace::State,

    pub(crate) ro_dbs: ReadonlyDatabases,

    // FIXME: temporarily here until we make ro_dbs usable.
    pub(crate) do_not_use_dbs: Databases,

    pub meter: Meter,
    internal_client: InternalClient,

    pub(crate) auth_token_cache: Arc<parking_lot::RwLock<FifoCache<Permissions>>>,
    pub(crate) rules_cache: Arc<parking_lot::RwLock<FifoCache<Arc<AccessRuleList>>>>,
    pub(crate) jwt_verifier: Option<JwtVerifier>,

    pub(crate) time: Monotime,

    pub(crate) topic_publish_notifier: TopicPublishNotifier,
}

impl AppState {
    pub(crate) fn new(cfg: Configuration, time: Monotime, internal_client: InternalClient) -> Self {
        let persistent_db = cfg
            .persistent_db
            .database(time.clone(), "persistent")
            .expect("should be able to initialize persistent database");
        let ephemeral_db = cfg
            .ephemeral_db
            .database(time.clone(), "ephemeral")
            .expect("should be able to initialize ephemeral database");

        let dbs = Databases::new(persistent_db, ephemeral_db);
        let ro_dbs = dbs.readonly();

        let namespace_state =
            diom_namespace::State::init(dbs.clone()).expect("initializing namespace state");

        let meter = opentelemetry::global::meter("diom.svix.com");

        let mut listen_addr = cfg.listen_address;
        if listen_addr.ip().is_unspecified() {
            listen_addr.set_ip(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
        }

        let auth_token_cache = Arc::new(parking_lot::RwLock::new(FifoCache::new(10_000)));
        let rules_cache = Arc::new(parking_lot::RwLock::new(FifoCache::new(1_000)));

        let jwt_verifier = JwtVerifier::try_new(&cfg.jwt).expect("invalid JWT configuration");

        AppState {
            cfg,
            namespace_state,
            ro_dbs,
            do_not_use_dbs: dbs,
            meter,
            internal_client,
            auth_token_cache,
            rules_cache,
            jwt_verifier,
            time,
            topic_publish_notifier: TopicPublishNotifier::new(),
        }
    }

    /// Make an internal call to a specific op id
    pub async fn internal_call<T: Serialize, U: DeserializeOwned>(
        &self,
        op_id: &'static str,
        body: &T,
    ) -> Result<U, InternalRequestError> {
        let path = format!("/api/{op_id}");
        self.internal_client.post(&path, body).await
    }
}
