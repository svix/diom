package diom_models

// This file is @generated DO NOT EDIT

type ClusterForceNodeUpgradeOut struct {
	InitialNodeState ServerState `msgpack:"initial_node_state"`
	FinalNodeState   ServerState `msgpack:"final_node_state"`
}
