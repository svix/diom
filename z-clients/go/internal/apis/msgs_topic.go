package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
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
	topic string,
	msgTopicConfigureIn coyote_models.MsgTopicConfigureIn,
) (*coyote_models.MsgTopicConfigureOut, error) {
	body := coyote_models.MsgTopicConfigureIn_{
		Namespace:  msgTopicConfigureIn.Namespace,
		Topic:      topic,
		Partitions: msgTopicConfigureIn.Partitions,
	}

	return coyote_proto.ExecuteRequest[coyote_models.MsgTopicConfigureIn_, coyote_models.MsgTopicConfigureOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1.msgs.topic.configure",
		&body,
	)
}
