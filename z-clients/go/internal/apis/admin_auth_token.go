package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.svix.com/go/diom/internal/models"
	diom_proto "diom.svix.com/go/diom/internal/proto"
)

type AdminAuthToken struct {
	client *diom_proto.HttpClient
}

func NewAdminAuthToken(client *diom_proto.HttpClient) AdminAuthToken {
	return AdminAuthToken{client}
}

// Create an auth token
func (adminAuthToken AdminAuthToken) Create(
	ctx context.Context,
	adminAuthTokenCreateIn diom_models.AdminAuthTokenCreateIn,
) (*diom_models.AdminAuthTokenCreateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAuthTokenCreateIn, diom_models.AdminAuthTokenCreateOut](
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
	adminAuthTokenExpireIn diom_models.AdminAuthTokenExpireIn,
) (*diom_models.AdminAuthTokenExpireOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAuthTokenExpireIn, diom_models.AdminAuthTokenExpireOut](
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
	adminAuthTokenRotateIn diom_models.AdminAuthTokenRotateIn,
) (*diom_models.AdminAuthTokenRotateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAuthTokenRotateIn, diom_models.AdminAuthTokenRotateOut](
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
	adminAuthTokenDeleteIn diom_models.AdminAuthTokenDeleteIn,
) (*diom_models.AdminAuthTokenDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAuthTokenDeleteIn, diom_models.AdminAuthTokenDeleteOut](
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
	adminAuthTokenListIn diom_models.AdminAuthTokenListIn,
) (*diom_models.ListResponseAdminAuthTokenOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAuthTokenListIn, diom_models.ListResponseAdminAuthTokenOut](
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
	adminAuthTokenUpdateIn diom_models.AdminAuthTokenUpdateIn,
) (*diom_models.AdminAuthTokenUpdateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAuthTokenUpdateIn, diom_models.AdminAuthTokenUpdateOut](
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
	adminAuthTokenWhoamiIn diom_models.AdminAuthTokenWhoamiIn,
) (*diom_models.AdminAuthTokenWhoamiOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAuthTokenWhoamiIn, diom_models.AdminAuthTokenWhoamiOut](
		ctx,
		adminAuthToken.client,
		"POST",
		"/api/v1.admin.auth-token.whoami",
		&adminAuthTokenWhoamiIn,
	)
}
