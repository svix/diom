package diom_models

// This file is @generated DO NOT EDIT

type NodeStatusOut struct {
	// A unique ID representing this node.
	//
	// This will never change unless the node is erased and reset
	NodeId                string      `msgpack:"node_id"`
	Address               string      `msgpack:"address"`                            // The advertised inter-server (cluster) address of this node.
	State                 ServerState `msgpack:"state"`                              // The last known state of this node
	LastCommittedLogIndex *uint64     `msgpack:"last_committed_log_index,omitempty"` // The index of the last log applied on this node
	LastCommittedTerm     *uint64     `msgpack:"last_committed_term,omitempty"`      // The raft term of the last committed leadership
}
