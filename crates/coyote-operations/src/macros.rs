#[macro_export]
macro_rules! raft_module_operations {
    (
        state = $state_type:ty,
        $trait_name:ident,
        $operation_class_name:ident {
            $(
                $variant:ident($operation:ident) -> $response_data_type:tt
            ),* $(,)?
        }
    ) => {
        ::paste::paste! {
            trait $trait_name: Into<$operation_class_name> + $crate::OperationRequest
            where
                Self: 'static,
            {
                fn apply(self, state: $state_type) -> Self::Response;
            }

            $(
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct [<$variant Response>](pub $crate::Result<$response_data_type>);
            )*

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum $operation_class_name {
                $(
                    $variant($operation),
                )*
            }

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Response {
                $(
                    $variant([<$variant Response>]),
                )*
            }

            impl coyote_operations::ModuleResponse for Response {}

            impl coyote_operations::ModuleRequest for $operation_class_name {
                type Response = Response;
            }

            $(
                impl From<$operation> for $operation_class_name {
                    fn from(value: $operation) -> Self {
                        Self::$variant(value)
                    }
                }
            )*

            $(
                impl TryFrom<Response> for [<$variant Response>] {
                    type Error = ();

                    fn try_from(value: Response) -> std::result::Result<Self, Self::Error> {
                        match value {
                            Response::$variant(value) => Ok(value),
                            _ => Err(()),
                        }
                    }
                }
            )*

            $(
                impl $crate::OperationResponse for [<$variant Response>] {
                    type ResponseParent = Response;
                }

                impl $crate::OperationRequest for $operation {
                    type Response = [<$variant Response>];
                    type RequestParent = $operation_class_name;
                }
            )*

            impl $operation_class_name {
                pub fn apply(self, state: $state_type) -> Response {
                    match self {
                        $(Self::$variant(req) => Response::$variant(req.apply(state)),)*
                    }
                }
            }
        }
    };
}
