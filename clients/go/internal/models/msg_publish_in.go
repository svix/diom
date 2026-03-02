package diom_models

// This file is @generated DO NOT EDIT

type MsgPublishIn struct {
	Topic string  `json:"topic"`
	Msgs  []MsgIn `json:"msgs"`
}
