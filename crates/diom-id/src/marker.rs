pub trait IdMarker {}

pub trait PublicIdMarker: IdMarker {
    const PREFIX: &str;
}

macro_rules! id_marker {
    ( $name:ident $(, $prefix:literal)? $(,)? ) => {
        pub enum $name {}

        impl $crate::marker::IdMarker for $name {}

        impl PartialEq for $name {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }

        impl Eq for $name {}

        $(
            impl $crate::marker::PublicIdMarker for $name {
                const PREFIX: &str = $prefix;
            }
        )?
    };
}
