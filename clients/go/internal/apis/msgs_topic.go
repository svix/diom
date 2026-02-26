package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type MsgsTopic struct {
	client *diom_proto.HttpClient
}

func NewMsgsTopic(client *diom_proto.HttpClient) MsgsTopic {
	return MsgsTopic{client}
}

// Upserts a new message topic with the given name.
func (msgsTopic MsgsTopic) Create(
	ctx context.Context,
	createMsgTopicIn diom_models.CreateMsgTopicIn,
) (*diom_models.CreateMsgTopicOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CreateMsgTopicIn, diom_models.CreateMsgTopicOut](
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
	getMsgTopicIn diom_models.GetMsgTopicIn,
) (*diom_models.GetMsgTopicOut, error) {
	return diom_proto.ExecuteRequest[diom_models.GetMsgTopicIn, diom_models.GetMsgTopicOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1/msgs/topic/get",
		nil,
		nil,
		&getMsgTopicIn,
	)
}
