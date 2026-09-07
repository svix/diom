package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.com/go/diom/internal/types"
)

type ClusterStatusOut struct {
	// The unique ID of this cluster.
	//
	// This value is populated on cluster initialization and will never change.
	ClusterId      *string `msgpack:"cluster_id,omitempty"`
	FeatureVersion uint32  `msgpack:"feature_version"` // The cluster's committed feature version, as seen by the node servicing this request.
	// The name of this cluster (as defined in the config)
	//
	// This value is not replicated and should only be used for debugging.
	ClusterName                    *string                `msgpack:"cluster_name,omitempty"`
	ThisNodeId                     string                 `msgpack:"this_node_id"`                               // The unique ID of the node servicing this request
	ThisNodeState                  ServerState            `msgpack:"this_node_state"`                            // The cluster state of the node servicing this request
	ThisNodeLastCommittedTimestamp uint64                 `msgpack:"this_node_last_committed_timestamp"`         // The timestamp of the last transaction committed on this node
	ThisNodeLastSnapshotId         *string                `msgpack:"this_node_last_snapshot_id,omitempty"`       // The last snapshot taken on this node
	ThisNodeLastPurgedLogIndex     *uint64                `msgpack:"this_node_last_purged_log_index,omitempty"`  // The last-purged log on this node
	ThisNodeLeaderLastContact      *diom_types.DurationMs `msgpack:"this_node_leader_last_contact_ms,omitempty"` // The duration since the last time we heard from the leader
	Nodes                          []NodeStatusOut        `msgpack:"nodes"`                                      // A list of all nodes known to be in the cluster
}
