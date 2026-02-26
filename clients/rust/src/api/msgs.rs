// this file is @generated
use super::MsgsTopic;
use crate::Configuration;

pub struct Msgs<'a> {
    cfg: &'a Configuration,
}

impl<'a> Msgs<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn topic(&self) -> MsgsTopic<'a> {
        MsgsTopic::new(self.cfg)
    }
}
