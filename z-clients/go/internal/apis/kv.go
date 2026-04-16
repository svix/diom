package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.svix.com/go/diom/internal/models"
	diom_proto "diom.svix.com/go/diom/internal/proto"
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
	key string,
	value []uint8,
	kvSetIn diom_models.KvSetIn,
) (*diom_models.KvSetOut, error) {
	body := diom_models.KvSetIn_{
		Namespace: kvSetIn.Namespace,
		Key:       key,
		Value:     value,
		Ttl:       kvSetIn.Ttl,
		Behavior:  kvSetIn.Behavior,
		Version:   kvSetIn.Version,
	}

	return diom_proto.ExecuteRequest[diom_models.KvSetIn_, diom_models.KvSetOut](
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
	kvGetIn diom_models.KvGetIn,
) (*diom_models.KvGetOut, error) {
	body := diom_models.KvGetIn_{
		Namespace:   kvGetIn.Namespace,
		Key:         key,
		Consistency: kvGetIn.Consistency,
	}

	return diom_proto.ExecuteRequest[diom_models.KvGetIn_, diom_models.KvGetOut](
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
	kvDeleteIn diom_models.KvDeleteIn,
) (*diom_models.KvDeleteOut, error) {
	body := diom_models.KvDeleteIn_{
		Namespace: kvDeleteIn.Namespace,
		Key:       key,
		Version:   kvDeleteIn.Version,
	}

	return diom_proto.ExecuteRequest[diom_models.KvDeleteIn_, diom_models.KvDeleteOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1.kv.delete",
		&body,
	)
}
