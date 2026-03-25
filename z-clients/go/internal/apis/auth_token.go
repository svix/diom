package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type AuthToken struct {
	client *diom_proto.HttpClient
}

func NewAuthToken(client *diom_proto.HttpClient) AuthToken {
	return AuthToken{client}
}

func (authToken AuthToken) Namespace() AuthTokenNamespace {
	return NewAuthTokenNamespace(authToken.client)
}

// Create Auth Token
func (authToken AuthToken) Create(
	ctx context.Context,
	authTokenCreateIn diom_models.AuthTokenCreateIn,
) (*diom_models.AuthTokenCreateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenCreateIn, diom_models.AuthTokenCreateOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1.auth-token.create",
		&authTokenCreateIn,
	)
}

// Expire Auth Token
func (authToken AuthToken) Expire(
	ctx context.Context,
	authTokenExpireIn diom_models.AuthTokenExpireIn,
) (*diom_models.AuthTokenExpireOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenExpireIn, diom_models.AuthTokenExpireOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1.auth-token.expire",
		&authTokenExpireIn,
	)
}

// Delete Auth Token
func (authToken AuthToken) Delete(
	ctx context.Context,
	authTokenDeleteIn diom_models.AuthTokenDeleteIn,
) (*diom_models.AuthTokenDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenDeleteIn, diom_models.AuthTokenDeleteOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1.auth-token.delete",
		&authTokenDeleteIn,
	)
}

// Verify Auth Token
func (authToken AuthToken) Verify(
	ctx context.Context,
	authTokenVerifyIn diom_models.AuthTokenVerifyIn,
) (*diom_models.AuthTokenVerifyOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenVerifyIn, diom_models.AuthTokenVerifyOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1.auth-token.verify",
		&authTokenVerifyIn,
	)
}

// List Auth Tokens
func (authToken AuthToken) List(
	ctx context.Context,
	authTokenListIn diom_models.AuthTokenListIn,
) (*diom_models.ListResponseAuthTokenOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenListIn, diom_models.ListResponseAuthTokenOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1.auth-token.list",
		&authTokenListIn,
	)
}

// Update Auth Token
func (authToken AuthToken) Update(
	ctx context.Context,
	authTokenUpdateIn diom_models.AuthTokenUpdateIn,
) (*diom_models.AuthTokenUpdateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenUpdateIn, diom_models.AuthTokenUpdateOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1.auth-token.update",
		&authTokenUpdateIn,
	)
}

// Rotate Auth Token
func (authToken AuthToken) Rotate(
	ctx context.Context,
	authTokenRotateIn diom_models.AuthTokenRotateIn,
) (*diom_models.AuthTokenRotateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AuthTokenRotateIn, diom_models.AuthTokenRotateOut](
		ctx,
		authToken.client,
		"POST",
		"/api/v1.auth-token.rotate",
		&authTokenRotateIn,
	)
}
