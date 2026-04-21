use crate::{AccessRuleList, Context, RequestedOperation};

pub struct Forbidden;

pub fn verify_operation(
    operation: &RequestedOperation<'_>,
    rules: &AccessRuleList,
    context: Context<'_>,
) -> Result<(), Forbidden> {
    for rule in &rules.deny {
        // deny rules take precedence, if we found a matching one
        // we can stop going through the rest and reject.
        if rule.matches(operation, context) {
            return Err(Forbidden);
        }
    }

    for rule in &rules.allow {
        // found an allow rule and allow deny rules have been checked.
        // request is okay.
        if rule.matches(operation, context) {
            return Ok(());
        }
    }

    // no deny or allow rules found => implicit deny
    Err(Forbidden)
}
