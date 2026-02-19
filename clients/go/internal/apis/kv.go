package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
	coyote_models "github.com/svix/coyote/clients/go/internal/models"
)


type Kv struct {
	client *coyote_proto.HttpClient
}

func NewKv(client *coyote_proto.HttpClient) Kv {
	return Kv{client}
}


type KvSetOptions struct {
		IdempotencyKey *string
		}
	
type KvGetOptions struct {
		IdempotencyKey *string
		}
	
type KvDeleteOptions struct {
		IdempotencyKey *string
		}
	
	// KV Set
	func (kv *Kv) Set(
	ctx context.Context,
	kvSetIn coyote_models.KvSetIn,
	o *KvSetOptions,
	) (*coyote_models.KvSetOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
			if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.KvSetIn,coyote_models.KvSetOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1/kv/set",
		nil,
		nil,
		headerMap,
		&kvSetIn,
		)
	}
	

	// KV Get
	func (kv *Kv) Get(
	ctx context.Context,
	kvGetIn coyote_models.KvGetIn,
	o *KvGetOptions,
	) (*coyote_models.KvGetOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
			if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.KvGetIn,coyote_models.KvGetOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1/kv/get",
		nil,
		nil,
		headerMap,
		&kvGetIn,
		)
	}
	

	// KV Delete
	func (kv *Kv) Delete(
	ctx context.Context,
	kvDeleteIn coyote_models.KvDeleteIn,
	o *KvDeleteOptions,
	) (*coyote_models.KvDeleteOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
			if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.KvDeleteIn,coyote_models.KvDeleteOut](
		ctx,
		kv.client,
		"POST",
		"/api/v1/kv/delete",
		nil,
		nil,
		headerMap,
		&kvDeleteIn,
		)
	}
	
