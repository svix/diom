package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type AdminAuthRole struct {
	client *diom_proto.HttpClient
}

func NewAdminAuthRole(client *diom_proto.HttpClient) AdminAuthRole {
	return AdminAuthRole{client}
}

// Create or update a role
func (adminAuthRole AdminAuthRole) Upsert(
	ctx context.Context,
	adminRoleUpsertIn diom_models.AdminRoleUpsertIn,
) (*diom_models.AdminRoleUpsertOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminRoleUpsertIn, diom_models.AdminRoleUpsertOut](
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
	adminRoleDeleteIn diom_models.AdminRoleDeleteIn,
) (*diom_models.AdminRoleDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminRoleDeleteIn, diom_models.AdminRoleDeleteOut](
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
	adminRoleGetIn diom_models.AdminRoleGetIn,
) (*diom_models.AdminRoleOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminRoleGetIn, diom_models.AdminRoleOut](
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
	adminRoleListIn diom_models.AdminRoleListIn,
) (*diom_models.ListResponseAdminRoleOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminRoleListIn, diom_models.ListResponseAdminRoleOut](
		ctx,
		adminAuthRole.client,
		"POST",
		"/api/v1.admin.auth-role.list",
		&adminRoleListIn,
	)
}
