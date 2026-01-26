use openraft::{AnyError, NodeId as RaftNodeId, StorageError, StorageIOError};

macro_rules! build_openraft_storageio_error_helper {
    ($name:ident) => {
        paste::paste! {
            pub(crate) fn [< $name _err >]<N: RaftNodeId>(e: impl Into<AnyError>) -> StorageError<N> {
                StorageError::IO {
                    source: StorageIOError::$name(e.into())
                }
            }
        }
    };
}

build_openraft_storageio_error_helper!(read);
build_openraft_storageio_error_helper!(write);
build_openraft_storageio_error_helper!(read_logs);
build_openraft_storageio_error_helper!(write_logs);
build_openraft_storageio_error_helper!(read_vote);
build_openraft_storageio_error_helper!(write_vote);

pub(crate) fn write_snapshot_err<N: RaftNodeId>(e: impl Into<AnyError>) -> StorageError<N> {
    StorageError::IO {
        source: StorageIOError::write_snapshot(None, e.into()),
    }
}

pub(crate) fn read_snapshot_err<N: RaftNodeId>(e: impl Into<AnyError>) -> StorageError<N> {
    StorageError::IO {
        source: StorageIOError::read_snapshot(None, e.into()),
    }
}
