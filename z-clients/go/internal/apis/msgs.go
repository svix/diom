package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type Msgs struct {
	client *diom_proto.HttpClient
}

func NewMsgs(client *diom_proto.HttpClient) Msgs {
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
	msgPublishIn diom_models.MsgPublishIn,
) (*diom_models.MsgPublishOut, error) {
	body := diom_models.MsgPublishIn_{
		Namespace:      msgPublishIn.Namespace,
		Topic:          topic,
		Msgs:           msgPublishIn.Msgs,
		IdempotencyKey: msgPublishIn.IdempotencyKey,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgPublishIn_, diom_models.MsgPublishOut](
		ctx,
		msgs.client,
		"POST",
		"/api/v1.msgs.publish",
		&body,
	)
}
