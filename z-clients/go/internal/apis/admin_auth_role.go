package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type AdminAuthRole struct {
	client *coyote_proto.HttpClient
}

func NewAdminAuthRole(client *coyote_proto.HttpClient) AdminAuthRole {
	return AdminAuthRole{client}
}

// Create or update a role
func (adminAuthRole AdminAuthRole) Upsert(
	ctx context.Context,
	adminRoleUpsertIn coyote_models.AdminRoleUpsertIn,
) (*coyote_models.AdminRoleUpsertOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminRoleUpsertIn, coyote_models.AdminRoleUpsertOut](
		ctx,
		adminAuthRole.client,
		"POST",
		"/api/v1.admin.auth-role.upsert",
		&adminRoleUpsertIn,
	)
}

// Delete a role
func (adminAuthRole AdminAuthRole) Delete(
	ctx context.Context,
	adminRoleDeleteIn coyote_models.AdminRoleDeleteIn,
) (*coyote_models.AdminRoleDeleteOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminRoleDeleteIn, coyote_models.AdminRoleDeleteOut](
		ctx,
		adminAuthRole.client,
		"POST",
		"/api/v1.admin.auth-role.delete",
		&adminRoleDeleteIn,
	)
}

// Get a role by ID
func (adminAuthRole AdminAuthRole) Get(
	ctx context.Context,
	adminRoleGetIn coyote_models.AdminRoleGetIn,
) (*coyote_models.AdminRoleOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminRoleGetIn, coyote_models.AdminRoleOut](
		ctx,
		adminAuthRole.client,
		"POST",
		"/api/v1.admin.auth-role.get",
		&adminRoleGetIn,
	)
}

// List all roles
func (adminAuthRole AdminAuthRole) List(
	ctx context.Context,
	adminRoleListIn coyote_models.AdminRoleListIn,
) (*coyote_models.ListResponseAdminRoleOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminRoleListIn, coyote_models.ListResponseAdminRoleOut](
		ctx,
		adminAuthRole.client,
		"POST",
		"/api/v1.admin.auth-role.list",
		&adminRoleListIn,
	)
}
