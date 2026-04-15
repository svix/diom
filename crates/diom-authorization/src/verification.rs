use crate::{AccessRule, AccessRuleEffect, Context, RequestedOperation};

pub struct Forbidden;

pub fn verify_operation(
    operation: &RequestedOperation<'_>,
    rules: &[AccessRule],
    context: Context<'_>,
) -> Result<(), Forbidden> {
    // Fallback if no rule matches: reject
    let mut result = Err(Forbidden);

    for rule in rules {
        if rule.matches(operation, context) {
            match rule.effect {
                AccessRuleEffect::Allow => {
                    // found an allow rule, so set the result accordingly.
                    // still need to go through other rules, as any deny rule
                    // that also matches takes precedence.
                    result = Ok(());
                }
                AccessRuleEffect::Deny => {
                    // deny rules take precedence, if we found a matching one
                    // we can stop going through the rest and reject.
                    return Err(Forbidden);
                }
            }
        }
    }

    result
}
