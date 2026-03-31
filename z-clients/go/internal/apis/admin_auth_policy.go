package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type AdminAuthPolicy struct {
	client *diom_proto.HttpClient
}

func NewAdminAuthPolicy(client *diom_proto.HttpClient) AdminAuthPolicy {
	return AdminAuthPolicy{client}
}

// Create or update an access policy
func (adminAuthPolicy AdminAuthPolicy) Upsert(
	ctx context.Context,
	adminAccessPolicyUpsertIn diom_models.AdminAccessPolicyUpsertIn,
) (*diom_models.AdminAccessPolicyUpsertOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAccessPolicyUpsertIn, diom_models.AdminAccessPolicyUpsertOut](
		ctx,
		adminAuthPolicy.client,
		"POST",
		"/api/v1.admin.auth-policy.upsert",
		&adminAccessPolicyUpsertIn,
	)
}

// Delete an access policy
func (adminAuthPolicy AdminAuthPolicy) Delete(
	ctx context.Context,
	adminAccessPolicyDeleteIn diom_models.AdminAccessPolicyDeleteIn,
) (*diom_models.AdminAccessPolicyDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAccessPolicyDeleteIn, diom_models.AdminAccessPolicyDeleteOut](
		ctx,
		adminAuthPolicy.client,
		"POST",
		"/api/v1.admin.auth-policy.delete",
		&adminAccessPolicyDeleteIn,
	)
}

// Get an access policy by ID
func (adminAuthPolicy AdminAuthPolicy) Get(
	ctx context.Context,
	adminAccessPolicyGetIn diom_models.AdminAccessPolicyGetIn,
) (*diom_models.AdminAccessPolicyOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAccessPolicyGetIn, diom_models.AdminAccessPolicyOut](
		ctx,
		adminAuthPolicy.client,
		"POST",
		"/api/v1.admin.auth-policy.get",
		&adminAccessPolicyGetIn,
	)
}

// List all access policies
func (adminAuthPolicy AdminAuthPolicy) List(
	ctx context.Context,
	adminAccessPolicyListIn diom_models.AdminAccessPolicyListIn,
) (*diom_models.ListResponseAdminAccessPolicyOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AdminAccessPolicyListIn, diom_models.ListResponseAdminAccessPolicyOut](
		ctx,
		adminAuthPolicy.client,
		"POST",
		"/api/v1.admin.auth-policy.list",
		&adminAccessPolicyListIn,
	)
}
