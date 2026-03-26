use coyote_authorization::RequestedOperation;

pub trait RequestInput {
    fn access_metadata(&self) -> AccessMetadata<'_>;
}

pub enum AccessMetadata<'a> {
    // FIXME: Get rid of this in favor of proper access metadata for everything
    AdminOnly,
    RuleProtected(RequestedOperation<'a>),
}
