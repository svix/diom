package diom_models

// This file is @generated DO NOT EDIT

type MsgPublishIn struct {
	Namespace *string `json:"namespace,omitempty"`
	Msgs      []MsgIn `json:"msgs"`
}

type MsgPublishIn_ struct {
	Namespace *string `json:"namespace,omitempty"`
	Topic     string  `json:"topic"`
	Msgs      []MsgIn `json:"msgs"`
}
