package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type ClusterForceSnapshotOut struct {
	SnapshotTime     time.Time `msgpack:"snapshot_time"`         // The wall-clock time at which the snapshot was initiated
	SnapshotLogIndex uint64    `msgpack:"snapshot_log_index"`    // The log index at which the snapshot was initiated
	SnapshotId       *string   `msgpack:"snapshot_id,omitempty"` // If this is `null`, the snapshot is still building in the background
}
