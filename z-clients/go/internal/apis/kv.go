package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
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
	key string,
	value []uint8,
	kvSetIn coyote_models.KvSetIn,
) (*coyote_models.KvSetOut, error) {
	body := coyote_models.KvSetIn_{
		Namespace: kvSetIn.Namespace,
		Key:       key,
		Value:     value,
		TtlMs:     kvSetIn.TtlMs,
		Behavior:  kvSetIn.Behavior,
		Version:   kvSetIn.Version,
	}

	return coyote_proto.ExecuteRequest[coyote_models.KvSetIn_, coyote_models.KvSetOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1.kv.set",
		&body,
	)
}

// KV Get
func (kv Kv) Get(
	ctx context.Context,
	key string,
	kvGetIn coyote_models.KvGetIn,
) (*coyote_models.KvGetOut, error) {
	body := coyote_models.KvGetIn_{
		Namespace:   kvGetIn.Namespace,
		Key:         key,
		Consistency: kvGetIn.Consistency,
	}

	return coyote_proto.ExecuteRequest[coyote_models.KvGetIn_, coyote_models.KvGetOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1.kv.get",
		&body,
	)
}

// KV Delete
func (kv Kv) Delete(
	ctx context.Context,
	key string,
	kvDeleteIn coyote_models.KvDeleteIn,
) (*coyote_models.KvDeleteOut, error) {
	body := coyote_models.KvDeleteIn_{
		Namespace: kvDeleteIn.Namespace,
		Key:       key,
		Version:   kvDeleteIn.Version,
	}

	return coyote_proto.ExecuteRequest[coyote_models.KvDeleteIn_, coyote_models.KvDeleteOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1.kv.delete",
		&body,
	)
}
