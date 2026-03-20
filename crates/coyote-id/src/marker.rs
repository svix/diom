pub trait IdMarker {
    type Use;
}

/// Marker type for internal-only ID markers.
///
/// (yes, the marker-ception serves a purpose)
pub enum InternalUse {}

/// Marker type for pubically-used ID markers.
///
/// (yes, the marker-ception serves a purpose)
pub enum PublicUse {}

pub trait PublicIdMarker: IdMarker<Use = PublicUse> {
    const PREFIX: &str;
}

macro_rules! id_marker {
    ( $name:ident $(,)? ) => {
        id_marker!(@impl $name, InternalUse);
    };
    ( $name:ident, $prefix:literal $(,)? ) => {
        id_marker!(@impl $name, PublicUse);

        impl $crate::marker::PublicIdMarker for $name {
            const PREFIX: &str = $prefix;
        }
    };
    ( @impl $name:ident, $use:ident ) => {
        pub enum $name {}

        impl $crate::marker::IdMarker for $name {
            type Use = $crate::marker::$use;
        }
    }
}
