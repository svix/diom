package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Kv struct {
	client *diom_proto.HttpClient
}

func NewKv(client *diom_proto.HttpClient) Kv {
	return Kv{client}
}

func (kv Kv) Namespace() KvNamespace {
	return NewKvNamespace(kv.client)
}

// KV Set
func (kv Kv) Set(
	ctx context.Context,
	kvSetIn diom_models.KvSetIn,
) (*diom_models.KvSetOut, error) {
	return diom_proto.ExecuteRequest[diom_models.KvSetIn, diom_models.KvSetOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1/kv/set",
		nil,
		nil,
		&kvSetIn,
	)
}

// KV Get
func (kv Kv) Get(
	ctx context.Context,
	kvGetIn diom_models.KvGetIn,
) (*diom_models.KvGetOut, error) {
	return diom_proto.ExecuteRequest[diom_models.KvGetIn, diom_models.KvGetOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1/kv/get",
		nil,
		nil,
		&kvGetIn,
	)
}

// KV Delete
func (kv Kv) Delete(
	ctx context.Context,
	kvDeleteIn diom_models.KvDeleteIn,
) (*diom_models.KvDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.KvDeleteIn, diom_models.KvDeleteOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1/kv/delete",
		nil,
		nil,
		&kvDeleteIn,
	)
}
