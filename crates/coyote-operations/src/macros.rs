#[macro_export]
macro_rules! raft_module_operations {
    (
      $trait_name:ident,
      $module_op_name:ident {
        $(
          $variant:ident($operation:ident) -> $response_data_type:tt
        ),* $(,)?
      },
      state = $state_type:ty $(,)?
    ) => {
        $crate::raft_module_operations!(
            $trait_name,
            $module_op_name {
                $($variant($operation) -> $response_data_type),*
            },
            state = $state_type,
            response = Response,
        );
    };

    (
      $trait_name:ident,
      $module_op_name:ident {
        $(
          $variant:ident($operation:ident) -> $response_data_type:tt
        ),* $(,)?
      },
      state = $state_type:ty,
      response = $response_name:ident $(,)?
    ) => {
        $crate::__reexports::paste::paste! {
            trait $trait_name: Into<$module_op_name> + $crate::OperationRequest
            where
                Self: 'static,
            {
                fn apply(self, state: $state_type, timestamp: jiff::Timestamp) -> Self::Response;
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
            pub enum $response_name {
                $(
                    $variant([<$variant Response>]),
                )*
            }

            impl coyote_operations::ModuleResponse for $response_name {}

            impl coyote_operations::ModuleRequest for $module_op_name {
                type Response = $response_name;
            }

            $(
                impl From<$operation> for $module_op_name {
                    fn from(value: $operation) -> Self {
                        Self::$variant(value)
                    }
                }
            )*

            $(
                impl TryFrom<$response_name> for [<$variant Response>] {
                    type Error = ();

                    fn try_from(value: $response_name) -> std::result::Result<Self, Self::Error> {
                        match value {
                            $response_name::$variant(value) => Ok(value),
                            _ => Err(()),
                        }
                    }
                }
            )*

            $(
                impl $crate::OperationResponse for [<$variant Response>] {
                    type ResponseParent = $response_name;
                }

                impl $crate::OperationRequest for $operation {
                    type Response = [<$variant Response>];
                    type RequestParent = $module_op_name;
                }
            )*

            impl $module_op_name {
                pub fn apply(self, state: $state_type, timestamp: jiff::Timestamp) -> $response_name {
                    match self {
                        $(Self::$variant(req) => $response_name::$variant(req.apply(state, timestamp)),)*
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! async_raft_module_operations {
    (
      $trait_name:ident,
      $module_op_name:ident {
        $(
          $variant:ident($operation:ident) -> $response_data_type:tt
        ),* $(,)?
      },
      state = $state_type:ty $(,)?
    ) => {
        $crate::async_raft_module_operations!(
            $trait_name,
            $module_op_name {
                $($variant($operation) -> $response_data_type),*
            },
            state = $state_type,
            response = Response,
        );
    };

    (
      $trait_name:ident,
      $module_op_name:ident {
        $(
          $variant:ident($operation:ident) -> $response_data_type:tt
        ),* $(,)?
      },
      state = $state_type:ty,
      response = $response_name:ident $(,)?
    ) => {
        $crate::__reexports::paste::paste! {
            trait $trait_name: Into<$module_op_name> + $crate::OperationRequest
            where
                Self: 'static,
            {
                async fn apply(self, state: $state_type, timestamp: jiff::Timestamp) -> Self::Response;
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
            pub enum $response_name {
                $(
                    $variant([<$variant Response>]),
                )*
            }

            impl coyote_operations::ModuleResponse for $response_name {}

            impl coyote_operations::ModuleRequest for $module_op_name {
                type Response = $response_name;
            }

            $(
                impl From<$operation> for $module_op_name {
                    fn from(value: $operation) -> Self {
                        Self::$variant(value)
                    }
                }
            )*

            $(
                impl TryFrom<$response_name> for [<$variant Response>] {
                    type Error = ();

                    fn try_from(value: $response_name) -> std::result::Result<Self, Self::Error> {
                        match value {
                            $response_name::$variant(value) => Ok(value),
                            _ => Err(()),
                        }
                    }
                }
            )*

            $(
                impl $crate::OperationResponse for [<$variant Response>] {
                    type ResponseParent = $response_name;
                }

                impl $crate::OperationRequest for $operation {
                    type Response = [<$variant Response>];
                    type RequestParent = $module_op_name;
                }
            )*

            impl $module_op_name {
                pub async fn apply(self, state: $state_type, timestamp: jiff::Timestamp) -> $response_name {
                    match self {
                        $(Self::$variant(req) => $response_name::$variant(req.apply(state, timestamp).await),)*
                    }
                }
            }
        }
    };
}
