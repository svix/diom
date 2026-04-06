pub mod policy;
pub mod role;
pub mod token;

use aide::axum::ApiRouter;
use coyote_authorization::AccessRule;
use validator::ValidationError;

use crate::AppState;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(token::router())
        .merge(role::router())
        .merge(policy::router())
}

fn validate_access_rule_list(list: &[AccessRule]) -> Result<(), ValidationError> {
    if let Some(pos) = list.iter().position(|rule| rule.uses_reserved_namespace()) {
        return Err(ValidationError::new("reserved_namespace").with_message(
            format!("access rule {} refers to a reserved namespace", pos + 1).into(),
        ));
    }

    Ok(())
}
