package diom_models

// This file is @generated DO NOT EDIT

type MsgTopicConfigureIn struct {
	Partitions uint16 `json:"partitions"`
}

type MsgTopicConfigureIn_ struct {
	Topic      string `json:"topic"`
	Partitions uint16 `json:"partitions"`
}
