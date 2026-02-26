package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type MsgsTopic struct {
	client *coyote_proto.HttpClient
}

func NewMsgsTopic(client *coyote_proto.HttpClient) MsgsTopic {
	return MsgsTopic{client}
}

// Upserts a new message topic with the given name.
func (msgsTopic MsgsTopic) Create(
	ctx context.Context,
	createMsgTopicIn coyote_models.CreateMsgTopicIn,
) (*coyote_models.CreateMsgTopicOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CreateMsgTopicIn, coyote_models.CreateMsgTopicOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1/msgs/topic/create",
		nil,
		nil,
		&createMsgTopicIn,
	)
}

// Get message topic with given name.
func (msgsTopic MsgsTopic) Get(
	ctx context.Context,
	getMsgTopicIn coyote_models.GetMsgTopicIn,
) (*coyote_models.GetMsgTopicOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.GetMsgTopicIn, coyote_models.GetMsgTopicOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1/msgs/topic/get",
		nil,
		nil,
		&getMsgTopicIn,
	)
}
