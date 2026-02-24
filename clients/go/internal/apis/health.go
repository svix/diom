package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Health struct {
	client *coyote_proto.HttpClient
}

func NewHealth(client *coyote_proto.HttpClient) Health {
	return Health{client}
}

// Verify the server is up and running.
func (health Health) Ping(
	ctx context.Context,
) (*coyote_models.PingOut, error) {
	return coyote_proto.ExecuteRequest[any, coyote_models.PingOut](
		ctx,
		health.client,
		"GET",
		"/api/v1/health/ping",
		nil,
		nil,
		nil,
	)
}
