package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Health struct {
	client *diom_proto.HttpClient
}

func NewHealth(client *diom_proto.HttpClient) Health {
	return Health{client}
}

// Verify the server is up and running.
func (health Health) Ping(
	ctx context.Context,
) (*diom_models.PingOut, error) {
	return diom_proto.ExecuteRequest[any, diom_models.PingOut](
		ctx,
		health.client,
		"GET",
		"/api/v1/health/ping",
		nil,
	)
}

// Intentionally return an error
func (health Health) Error(
	ctx context.Context,
) error {
	_, err := diom_proto.ExecuteRequest[any, any](
		ctx,
		health.client,
		"POST",
		"/api/v1/health/error",
		nil,
		nil,
		nil,
	)
	return err
}
