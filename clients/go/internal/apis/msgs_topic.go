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

// Configures the number of partitions for a topic.
//
// Partition count can only be increased, never decreased. The default for a new topic is 1.
func (msgsTopic MsgsTopic) Configure(
	ctx context.Context,
	msgTopicConfigureIn diom_models.MsgTopicConfigureIn,
) (*diom_models.MsgTopicConfigureOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgTopicConfigureIn, diom_models.MsgTopicConfigureOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1/msgs/topic/configure",
		nil,
		nil,
		&msgTopicConfigureIn,
	)
}
