package diom_models

// This file is @generated DO NOT EDIT

type MsgTopicConfigureIn struct {
	Namespace  *string `json:"namespace,omitempty"`
	Partitions uint16  `json:"partitions"`
}

type MsgTopicConfigureIn_ struct {
	Namespace  *string `json:"namespace,omitempty"`
	Topic      string  `json:"topic"`
	Partitions uint16  `json:"partitions"`
}
