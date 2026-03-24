package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type ClusterStatusOut struct {
	// The unique ID of this cluster.pub(crate)
	//
	// This value is populated on cluster initialization and will never change.
	ClusterId *string `msgpack:"cluster_id,omitempty"`
	// The name of this cluster (as defined in the config)
	//
	// This value is not replicated and should only be used for debugging.
	ClusterName                    *string         `msgpack:"cluster_name,omitempty"`
	ThisNodeId                     string          `msgpack:"this_node_id"`                       // The unique ID of the node servicing this request
	ThisNodeState                  ServerState     `msgpack:"this_node_state"`                    // The cluster state of the node servicing this request
	ThisNodeLastCommittedTimestamp time.Time       `msgpack:"this_node_last_committed_timestamp"` // The timestamp of the last transaction committed on this node
	Nodes                          []NodeStatusOut `msgpack:"nodes"`                              // A list of all nodes known to be in the cluster
}
