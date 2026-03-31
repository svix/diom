package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type AdminAuthPolicy struct {
	client *coyote_proto.HttpClient
}

func NewAdminAuthPolicy(client *coyote_proto.HttpClient) AdminAuthPolicy {
	return AdminAuthPolicy{client}
}

// Create or update an access policy
func (adminAuthPolicy AdminAuthPolicy) Upsert(
	ctx context.Context,
	adminAccessPolicyUpsertIn coyote_models.AdminAccessPolicyUpsertIn,
) (*coyote_models.AdminAccessPolicyUpsertOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAccessPolicyUpsertIn, coyote_models.AdminAccessPolicyUpsertOut](
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
	adminAccessPolicyDeleteIn coyote_models.AdminAccessPolicyDeleteIn,
) (*coyote_models.AdminAccessPolicyDeleteOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAccessPolicyDeleteIn, coyote_models.AdminAccessPolicyDeleteOut](
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
	adminAccessPolicyGetIn coyote_models.AdminAccessPolicyGetIn,
) (*coyote_models.AdminAccessPolicyOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAccessPolicyGetIn, coyote_models.AdminAccessPolicyOut](
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
	adminAccessPolicyListIn coyote_models.AdminAccessPolicyListIn,
) (*coyote_models.ListResponseAdminAccessPolicyOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AdminAccessPolicyListIn, coyote_models.ListResponseAdminAccessPolicyOut](
		ctx,
		adminAuthPolicy.client,
		"POST",
		"/api/v1.admin.auth-policy.list",
		&adminAccessPolicyListIn,
	)
}
