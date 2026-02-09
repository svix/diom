// use like raft_module_operations!(
//     RateLimiterOperation => [
//         (Limit, LimitOperation, LimitResponseData, LimitResponse),
//         (Reset, ResetOperation, (), ResetResponse)
//     ]
// );
#[macro_export]
macro_rules! raft_module_request_trait {
    ($trait_name:ident, $operation_class_name:ident, $state_type:ty) => {
        trait $trait_name: Into<$operation_class_name> + $crate::OperationRequest
        where
            Self: 'static,
        {
            fn apply(self, state: $state_type) -> Self::Response;
        }
    };
}

#[macro_export]
macro_rules! raft_module_operations {
        (
            $operation_class_name:ident => [
            $(
                ($variant:ident, $operation:ident, $response_data_type:tt, $response_type_name:ident)
            ),* $(,)?
            ]
        )
     => {
        $(
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct $response_type_name(pub $crate::Result<$response_data_type>);
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
              $variant($response_type_name),
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
            impl TryFrom<Response> for $response_type_name {
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
            impl coyote_operations::OperationResponse for $response_type_name {
                type ResponseParent = Response;
            }

            impl coyote_operations::OperationRequest for $operation {
                type Response = $response_type_name;
                type RequestParent = $operation_class_name;
            }
        )*
    }
}
