use diom_authorization::RequestedOperation;

pub trait RequestInput {
    fn access_metadata(&self) -> AccessMetadata<'_>;
    fn operation(&self) -> Option<RequestedOperation<'_>> {
        // Convenience operation. If we ever add more variants to AccessMetadata,
        // remove this and  have callers use access_metadata directly as appropriate.
        match self.access_metadata() {
            AccessMetadata::NoAuthorizationRequired => None,
            AccessMetadata::RuleProtected(op) => Some(op),
        }
    }
}

pub enum AccessMetadata<'a> {
    NoAuthorizationRequired,
    RuleProtected(RequestedOperation<'a>),
}
