package coyote_models

// This file is @generated DO NOT EDIT

type MsgPublishIn struct {
	Msgs []MsgIn `json:"msgs"`
}

type MsgPublishIn_ struct {
	Topic string  `json:"topic"`
	Msgs  []MsgIn `json:"msgs"`
}
