package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Msgs struct {
	client    *diom_proto.HttpClient
	Namespace *MsgsNamespace
	Queue     *MsgsQueue
	Stream    *MsgsStream
	Topic     *MsgsTopic
}

func NewMsgs(client *diom_proto.HttpClient) Msgs {
	return Msgs{client}
}

// Publishes messages to a topic within a namespace.
func (msgs Msgs) Publish(
	ctx context.Context,
	msgPublishIn diom_models.MsgPublishIn,
) (*diom_models.MsgPublishOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgPublishIn, diom_models.MsgPublishOut](
		ctx,
		msgs.client,
		"POST",
		"/api/v1/msgs/publish",
		nil,
		nil,
		&msgPublishIn,
	)
}
