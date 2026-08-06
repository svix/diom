package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type Health struct {
	client *diom_proto.HttpClient
}

func NewHealth(client *diom_proto.HttpClient) Health {
	return Health{client}
}

// Verify the server is up and running.
//
// This endpoint only checks the server itself, not the cluster mechanism, and should not be used
// as a readiness gate.
func (health Health) Ping(
	ctx context.Context,
) (*diom_models.PingOut, error) {
	return diom_proto.ExecuteRequest[any, diom_models.PingOut](
		ctx,
		health.client,
		"GET",
		"/api/v1.health.ping",
		nil,
	)
}

// Verify that this server is ready to serve customer traffic.
func (health Health) Ready(
	ctx context.Context,
) (*diom_models.ReadyOut, error) {
	return diom_proto.ExecuteRequest[any, diom_models.ReadyOut](
		ctx,
		health.client,
		"GET",
		"/api/v1.health.ready",
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
		"/api/v1.health.error",
		nil,
	)
	return err
}
