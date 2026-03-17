package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Kv struct {
	client *coyote_proto.HttpClient
}

func NewKv(client *coyote_proto.HttpClient) Kv {
	return Kv{client}
}

func (kv Kv) Namespace() KvNamespace {
	return NewKvNamespace(kv.client)
}

// KV Set
func (kv Kv) Set(
	ctx context.Context,
	kvSetIn coyote_models.KvSetIn,
) (*coyote_models.KvSetOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.KvSetIn, coyote_models.KvSetOut](
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
	kvGetIn coyote_models.KvGetIn,
) (*coyote_models.KvGetOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.KvGetIn, coyote_models.KvGetOut](
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
	kvDeleteIn coyote_models.KvDeleteIn,
) (*coyote_models.KvDeleteOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.KvDeleteIn, coyote_models.KvDeleteOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1/kv/delete",
		nil,
		nil,
		&kvDeleteIn,
	)
}
