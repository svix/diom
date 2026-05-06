package diom_models

// This file is @generated DO NOT EDIT

type ClusterForceElectionOut struct {
	PreviousLeaderId *string `msgpack:"previous_leader_id,omitempty"`
	NewLeaderId      *string `msgpack:"new_leader_id,omitempty"`
}
