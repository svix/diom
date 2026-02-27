package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Msgs struct {
	client    *coyote_proto.HttpClient
	Namespace *MsgsNamespace
	Stream    *MsgsStream
}

func NewMsgs(client *coyote_proto.HttpClient) Msgs {
	return Msgs{client}
}

// Publishes messages to a topic within a namespace.
func (msgs Msgs) Publish(
	ctx context.Context,
	publishIn coyote_models.PublishIn,
) (*coyote_models.PublishOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.PublishIn, coyote_models.PublishOut](
		ctx,
		msgs.client,
		"POST",
		"/api/v1/msgs/publish",
		nil,
		nil,
		&publishIn,
	)
}
