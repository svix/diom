package diom_models

// This file is @generated DO NOT EDIT

type MsgPublishIn struct {
	Msgs  []MsgIn `json:"msgs"`
	Topic string  `json:"topic"`
}
