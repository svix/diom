package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type KvNamespace struct {
	client *diom_proto.HttpClient
}

func NewKvNamespace(client *diom_proto.HttpClient) KvNamespace {
	return KvNamespace{client}
}

// Configure KV namespace
func (kvNamespace KvNamespace) Configure(
	ctx context.Context,
	kvConfigureNamespaceIn diom_models.KvConfigureNamespaceIn,
) (*diom_models.KvConfigureNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.KvConfigureNamespaceIn, diom_models.KvConfigureNamespaceOut](
		ctx,
		kvNamespace.client,
		"POST",
		"/api/v1.kv.namespace.configure",
		&kvConfigureNamespaceIn,
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
		"/api/v1.kv.namespace.get",
		&kvGetNamespaceIn,
	)
}
