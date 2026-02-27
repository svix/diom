// this file is @generated
use super::MsgsNamespace;
use crate::Configuration;

pub struct Msgs<'a> {
    cfg: &'a Configuration,
}

impl<'a> Msgs<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn namespace(&self) -> MsgsNamespace<'a> {
        MsgsNamespace::new(self.cfg)
    }
}
