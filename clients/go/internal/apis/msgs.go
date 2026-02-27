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
}

func NewMsgs(client *diom_proto.HttpClient) Msgs {
	return Msgs{client}
}

// Publishes messages to a topic within a namespace.
func (msgs Msgs) Publish(
	ctx context.Context,
	publishIn diom_models.PublishIn,
) (*diom_models.PublishOut, error) {
	return diom_proto.ExecuteRequest[diom_models.PublishIn, diom_models.PublishOut](
		ctx,
		msgs.client,
		"POST",
		"/api/v1/msgs/publish",
		nil,
		nil,
		&publishIn,
	)
}
