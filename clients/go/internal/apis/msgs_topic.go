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
	topicConfigureIn diom_models.TopicConfigureIn,
) (*diom_models.TopicConfigureOut, error) {
	return diom_proto.ExecuteRequest[diom_models.TopicConfigureIn, diom_models.TopicConfigureOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1/msgs/topic/configure",
		nil,
		nil,
		&topicConfigureIn,
	)
}
