package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Cache struct {
	client *coyote_proto.HttpClient
}

func NewCache(client *coyote_proto.HttpClient) Cache {
	return Cache{client}
}

type CacheSetOptions struct {
	IdempotencyKey *string
}

type CacheGetOptions struct {
	IdempotencyKey *string
}

type CacheDeleteOptions struct {
	IdempotencyKey *string
}

// Cache Set
func (cache *Cache) Set(
	ctx context.Context,
	cacheSetIn coyote_models.CacheSetIn,
	o *CacheSetOptions,
) (*coyote_models.CacheSetOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.CacheSetIn, coyote_models.CacheSetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/set",
		nil,
		nil,
		headerMap,
		&cacheSetIn,
	)
}

// Cache Get
func (cache *Cache) Get(
	ctx context.Context,
	cacheGetIn coyote_models.CacheGetIn,
	o *CacheGetOptions,
) (*coyote_models.CacheGetOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.CacheGetIn, coyote_models.CacheGetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/get",
		nil,
		nil,
		headerMap,
		&cacheGetIn,
	)
}

// Cache Delete
func (cache *Cache) Delete(
	ctx context.Context,
	cacheDeleteIn coyote_models.CacheDeleteIn,
	o *CacheDeleteOptions,
) (*coyote_models.CacheDeleteOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.CacheDeleteIn, coyote_models.CacheDeleteOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/delete",
		nil,
		nil,
		headerMap,
		&cacheDeleteIn,
	)
}
