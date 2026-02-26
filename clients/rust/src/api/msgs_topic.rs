// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsTopic<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsTopic<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Upserts a new message topic with the given name.
    pub async fn create(&self, create_msg_topic_in: CreateMsgTopicIn) -> Result<CreateMsgTopicOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/topic/create")
            .with_body_param(create_msg_topic_in)
            .execute(self.cfg)
            .await
    }

    /// Get message topic with given name.
    pub async fn get(&self, get_msg_topic_in: GetMsgTopicIn) -> Result<GetMsgTopicOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/topic/get")
            .with_body_param(get_msg_topic_in)
            .execute(self.cfg)
            .await
    }
}
