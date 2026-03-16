package coyote_models

// This file is @generated DO NOT EDIT

type NodeStatusOut struct {
	// A unique ID representing this node.
	//
	// This will never change unless the node is erased and reset
	NodeId                string      `json:"node_id"`
	Address               Node        `json:"address"`                            // The advertised inter-server (cluster) address of this node.
	State                 ServerState `json:"state"`                              // The last known state of this node
	LastCommittedLogIndex *uint64     `json:"last_committed_log_index,omitempty"` // The index of the last log applied on this node
	LastCommittedTerm     *uint64     `json:"last_committed_term,omitempty"`      // The raft term of the last committed leadership
}
