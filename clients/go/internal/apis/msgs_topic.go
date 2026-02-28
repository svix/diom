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

// Configures the number of partitions for a topic.
//
// Partition count can only be increased, never decreased. The default for a new topic is 1.
func (msgsTopic MsgsTopic) Configure(
	ctx context.Context,
	topicConfigureIn coyote_models.TopicConfigureIn,
) (*coyote_models.TopicConfigureOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.TopicConfigureIn, coyote_models.TopicConfigureOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1/msgs/topic/configure",
		nil,
		nil,
		&topicConfigureIn,
	)
}
