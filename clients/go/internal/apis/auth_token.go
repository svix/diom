package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type AuthToken struct {
	client *coyote_proto.HttpClient
}

func NewAuthToken(client *coyote_proto.HttpClient) AuthToken {
	return AuthToken{client}
}

func (authToken AuthToken) Namespace() AuthTokenNamespace {
	return NewAuthTokenNamespace(authToken.client)
}

// Create Auth Token
func (authToken AuthToken) Create(
	ctx context.Context,
	authTokenCreateIn coyote_models.AuthTokenCreateIn,
) (*coyote_models.AuthTokenCreateOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenCreateIn, coyote_models.AuthTokenCreateOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1/auth-token/create",
		&authTokenCreateIn,
	)
}

// Expire Auth Token
func (authToken AuthToken) Expire(
	ctx context.Context,
	authTokenExpireIn coyote_models.AuthTokenExpireIn,
) (*coyote_models.AuthTokenExpireOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenExpireIn, coyote_models.AuthTokenExpireOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1/auth-token/expire",
		&authTokenExpireIn,
	)
}

// Delete Auth Token
func (authToken AuthToken) Delete(
	ctx context.Context,
	authTokenDeleteIn coyote_models.AuthTokenDeleteIn,
) (*coyote_models.AuthTokenDeleteOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenDeleteIn, coyote_models.AuthTokenDeleteOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1/auth-token/delete",
		&authTokenDeleteIn,
	)
}

// Verify Auth Token
func (authToken AuthToken) Verify(
	ctx context.Context,
	authTokenVerifyIn coyote_models.AuthTokenVerifyIn,
) (*coyote_models.AuthTokenVerifyOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenVerifyIn, coyote_models.AuthTokenVerifyOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1/auth-token/verify",
		&authTokenVerifyIn,
	)
}

// List Auth Tokens
func (authToken AuthToken) List(
	ctx context.Context,
	authTokenListIn coyote_models.AuthTokenListIn,
) (*coyote_models.ListResponseAuthTokenOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenListIn, coyote_models.ListResponseAuthTokenOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1/auth-token/list",
		&authTokenListIn,
	)
}

// Update Auth Token
func (authToken AuthToken) Update(
	ctx context.Context,
	authTokenUpdateIn coyote_models.AuthTokenUpdateIn,
) (*coyote_models.AuthTokenUpdateOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenUpdateIn, coyote_models.AuthTokenUpdateOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1/auth-token/update",
		&authTokenUpdateIn,
	)
}

// Rotate Auth Token
func (authToken AuthToken) Rotate(
	ctx context.Context,
	authTokenRotateIn coyote_models.AuthTokenRotateIn,
) (*coyote_models.AuthTokenRotateOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AuthTokenRotateIn, coyote_models.AuthTokenRotateOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1/auth-token/rotate",
		&authTokenRotateIn,
	)
}
