#[macro_export]
macro_rules! raft_module_operations {
    (
      $trait_name:ident,
      $module_op_name:ident {
        $(
          $variant:ident($operation:ident) -> $response_data_type:tt
        ),* $(,)?
      },
      state = $state_type:ty,
    ) => {
        $crate::__reexports::paste::paste! {
            trait $trait_name: Into<$module_op_name> + $crate::OperationRequest
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
            pub enum $module_op_name {
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

            impl diom_operations::ModuleResponse for Response {}

            impl diom_operations::ModuleRequest for $module_op_name {
                type Response = Response;
            }

            $(
                impl From<$operation> for $module_op_name {
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
                    type RequestParent = $module_op_name;
                }
            )*

            impl $module_op_name {
                pub fn apply(self, state: $state_type) -> Response {
                    match self {
                        $(Self::$variant(req) => Response::$variant(req.apply(state)),)*
                    }
                }
            }
        }
    };
}
