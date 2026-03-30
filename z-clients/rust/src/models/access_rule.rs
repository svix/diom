// this file is @generated
use serde::{Deserialize, Serialize};

use super::{access_rule_effect::AccessRuleEffect, resource_pattern::ResourcePattern};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessRule {
    pub effect: AccessRuleEffect,

    pub resource: ResourcePattern,

    pub actions: Vec<String>,
}

impl AccessRule {
    pub fn new(effect: AccessRuleEffect, resource: ResourcePattern, actions: Vec<String>) -> Self {
        Self {
            effect,
            resource,
            actions,
        }
    }
}
