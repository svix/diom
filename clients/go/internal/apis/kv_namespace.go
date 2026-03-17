package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type KvNamespace struct {
	client *coyote_proto.HttpClient
}

func NewKvNamespace(client *coyote_proto.HttpClient) KvNamespace {
	return KvNamespace{client}
}

// Create KV namespace
func (kvNamespace KvNamespace) Create(
	ctx context.Context,
	kvCreateNamespaceIn coyote_models.KvCreateNamespaceIn,
) (*coyote_models.KvCreateNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.KvCreateNamespaceIn, coyote_models.KvCreateNamespaceOut](
		ctx,
		kvNamespace.client,
		"POST",
		"/api/v1/kv/namespace/create",
		&kvCreateNamespaceIn,
	)
}

// Get KV namespace
func (kvNamespace KvNamespace) Get(
	ctx context.Context,
	kvGetNamespaceIn coyote_models.KvGetNamespaceIn,
) (*coyote_models.KvGetNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.KvGetNamespaceIn, coyote_models.KvGetNamespaceOut](
		ctx,
		kvNamespace.client,
		"POST",
		"/api/v1/kv/namespace/get",
		&kvGetNamespaceIn,
	)
}
