package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type KvNamespace struct {
	client *diom_proto.HttpClient
}

func NewKvNamespace(client *diom_proto.HttpClient) KvNamespace {
	return KvNamespace{client}
}

// Create KV namespace
func (kvNamespace KvNamespace) Create(
	ctx context.Context,
	kvCreateNamespaceIn diom_models.KvCreateNamespaceIn,
) (*diom_models.KvCreateNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.KvCreateNamespaceIn, diom_models.KvCreateNamespaceOut](
		ctx,
		kvNamespace.client,
		"POST",
		"/api/v1/kv/namespace/create",
		nil,
		nil,
		&kvCreateNamespaceIn,
	)
}

// Get KV namespace
func (kvNamespace KvNamespace) Get(
	ctx context.Context,
	kvGetNamespaceIn diom_models.KvGetNamespaceIn,
) (*diom_models.KvGetNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.KvGetNamespaceIn, diom_models.KvGetNamespaceOut](
		ctx,
		kvNamespace.client,
		"POST",
		"/api/v1/kv/namespace/get",
		nil,
		nil,
		&kvGetNamespaceIn,
	)
}
