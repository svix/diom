use diom_authorization::RequestedOperation;

pub trait RequestInput {
    fn access_metadata(&self) -> AccessMetadata<'_>;
}

pub enum AccessMetadata<'a> {
    AdminOnly,
    RuleProtected(RequestedOperation<'a>),
}
