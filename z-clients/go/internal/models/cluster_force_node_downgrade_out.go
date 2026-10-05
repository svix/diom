package diom_models

// This file is @generated DO NOT EDIT

type ClusterForceNodeDowngradeOut struct {
	InitialNodeState ServerState `msgpack:"initial_node_state"`
	FinalNodeState   ServerState `msgpack:"final_node_state"`
}
