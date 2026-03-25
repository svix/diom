package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type AdminAuthToken struct {
	client *coyote_proto.HttpClient
}

func NewAdminAuthToken(client *coyote_proto.HttpClient) AdminAuthToken {
	return AdminAuthToken{client}
}

// Create an auth token
func (adminAuthToken AdminAuthToken) Create(
	ctx context.Context,
	adminAuthTokenCreateIn coyote_models.AdminAuthTokenCreateIn,
) (*coyote_models.AdminAuthTokenCreateOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAuthTokenCreateIn, coyote_models.AdminAuthTokenCreateOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.create",
		&adminAuthTokenCreateIn,
	)
}

// Expire an auth token
func (adminAuthToken AdminAuthToken) Expire(
	ctx context.Context,
	adminAuthTokenExpireIn coyote_models.AdminAuthTokenExpireIn,
) (*coyote_models.AdminAuthTokenExpireOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAuthTokenExpireIn, coyote_models.AdminAuthTokenExpireOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.expire",
		&adminAuthTokenExpireIn,
	)
}

// Rotate an auth token, invalidating the old one and issuing a new secret
func (adminAuthToken AdminAuthToken) Rotate(
	ctx context.Context,
	adminAuthTokenRotateIn coyote_models.AdminAuthTokenRotateIn,
) (*coyote_models.AdminAuthTokenRotateOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAuthTokenRotateIn, coyote_models.AdminAuthTokenRotateOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.rotate",
		&adminAuthTokenRotateIn,
	)
}

// Delete an auth token
func (adminAuthToken AdminAuthToken) Delete(
	ctx context.Context,
	adminAuthTokenDeleteIn coyote_models.AdminAuthTokenDeleteIn,
) (*coyote_models.AdminAuthTokenDeleteOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAuthTokenDeleteIn, coyote_models.AdminAuthTokenDeleteOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.delete",
		&adminAuthTokenDeleteIn,
	)
}

// List auth tokens for a given owner
func (adminAuthToken AdminAuthToken) List(
	ctx context.Context,
	adminAuthTokenListIn coyote_models.AdminAuthTokenListIn,
) (*coyote_models.ListResponseAdminAuthTokenOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAuthTokenListIn, coyote_models.ListResponseAdminAuthTokenOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.list",
		&adminAuthTokenListIn,
	)
}

// Update an auth token's properties
func (adminAuthToken AdminAuthToken) Update(
	ctx context.Context,
	adminAuthTokenUpdateIn coyote_models.AdminAuthTokenUpdateIn,
) (*coyote_models.AdminAuthTokenUpdateOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAuthTokenUpdateIn, coyote_models.AdminAuthTokenUpdateOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.update",
		&adminAuthTokenUpdateIn,
	)
}

// Return the role of the currently authenticated token
func (adminAuthToken AdminAuthToken) Whoami(
	ctx context.Context,
	adminAuthTokenWhoamiIn coyote_models.AdminAuthTokenWhoamiIn,
) (*coyote_models.AdminAuthTokenWhoamiOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAuthTokenWhoamiIn, coyote_models.AdminAuthTokenWhoamiOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.whoami",
		&adminAuthTokenWhoamiIn,
	)
}
