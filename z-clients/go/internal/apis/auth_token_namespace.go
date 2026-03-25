package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type AuthTokenNamespace struct {
	client *coyote_proto.HttpClient
}

func NewAuthTokenNamespace(client *coyote_proto.HttpClient) AuthTokenNamespace {
	return AuthTokenNamespace{client}
}

// Create Auth Token namespace
func (authTokenNamespace AuthTokenNamespace) Create(
	ctx context.Context,
	authTokenCreateNamespaceIn coyote_models.AuthTokenCreateNamespaceIn,
) (*coyote_models.AuthTokenCreateNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenCreateNamespaceIn, coyote_models.AuthTokenCreateNamespaceOut](
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
	authTokenGetNamespaceIn coyote_models.AuthTokenGetNamespaceIn,
) (*coyote_models.AuthTokenGetNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenGetNamespaceIn, coyote_models.AuthTokenGetNamespaceOut](
		ctx,
		authTokenNamespace.client,
		"POST",
		"/api/v1.auth-token.namespace.get",
		&authTokenGetNamespaceIn,
	)
}
