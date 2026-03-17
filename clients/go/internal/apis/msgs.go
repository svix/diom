package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Msgs struct {
	client *coyote_proto.HttpClient
}

func NewMsgs(client *coyote_proto.HttpClient) Msgs {
	return Msgs{client}
}

func (msgs Msgs) Namespace() MsgsNamespace {
	return NewMsgsNamespace(msgs.client)
}
func (msgs Msgs) Queue() MsgsQueue {
	return NewMsgsQueue(msgs.client)
}
func (msgs Msgs) Stream() MsgsStream {
	return NewMsgsStream(msgs.client)
}
func (msgs Msgs) Topic() MsgsTopic {
	return NewMsgsTopic(msgs.client)
}

// Publishes messages to a topic within a namespace.
func (msgs Msgs) Publish(
	ctx context.Context,
	topic string,
	msgPublishIn coyote_models.MsgPublishIn,
) (*coyote_models.MsgPublishOut, error) {
	body := coyote_models.MsgPublishIn_{
		Topic: topic,
		Msgs:  msgPublishIn.Msgs,
	}

	return coyote_proto.ExecuteRequest[coyote_models.MsgPublishIn_, coyote_models.MsgPublishOut](
		ctx,
		msgs.client,
		"POST",
		"/api/v1/msgs/publish",
		&body,
	)
}
