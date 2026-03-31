use coyote_authorization::RequestedOperation;

pub trait RequestInput {
    fn operation(&self) -> RequestedOperation<'_>;
}
