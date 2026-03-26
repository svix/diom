package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type ClusterForceSnapshotOut struct {
	SnapshotTime     time.Time `msgpack:"snapshot_time"`
	SnapshotLogIndex uint64    `msgpack:"snapshot_log_index"`
}
