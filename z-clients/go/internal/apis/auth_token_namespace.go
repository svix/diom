package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type AuthTokenNamespace struct {
	client *diom_proto.HttpClient
}

func NewAuthTokenNamespace(client *diom_proto.HttpClient) AuthTokenNamespace {
	return AuthTokenNamespace{client}
}

// Create Auth Token namespace
func (authTokenNamespace AuthTokenNamespace) Create(
	ctx context.Context,
	authTokenCreateNamespaceIn diom_models.AuthTokenCreateNamespaceIn,
) (*diom_models.AuthTokenCreateNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenCreateNamespaceIn, diom_models.AuthTokenCreateNamespaceOut](
		ctx,
		authTokenNamespace.client,
		"POST",
		"/api/v1.auth-token.namespace.create",
		&authTokenCreateNamespaceIn,
	)
}

// Get Auth Token namespace
func (authTokenNamespace AuthTokenNamespace) Get(
	ctx context.Context,
	authTokenGetNamespaceIn diom_models.AuthTokenGetNamespaceIn,
) (*diom_models.AuthTokenGetNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenGetNamespaceIn, diom_models.AuthTokenGetNamespaceOut](
		ctx,
		authTokenNamespace.client,
		"POST",
		"/api/v1.auth-token.namespace.get",
		&authTokenGetNamespaceIn,
	)
}
