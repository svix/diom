package coyote_models

// This file is @generated DO NOT EDIT

type TopicConfigureIn struct {
	Name       string `json:"name"`
	Partitions uint16 `json:"partitions"`
	Topic      string `json:"topic"`
}
