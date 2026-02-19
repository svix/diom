package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Cache struct {
	client *diom_proto.HttpClient
}

func NewCache(client *diom_proto.HttpClient) Cache {
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
	cacheSetIn diom_models.CacheSetIn,
	o *CacheSetOptions,
) (*diom_models.CacheSetOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.CacheSetIn, diom_models.CacheSetOut](
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
	cacheGetIn diom_models.CacheGetIn,
	o *CacheGetOptions,
) (*diom_models.CacheGetOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.CacheGetIn, diom_models.CacheGetOut](
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
	cacheDeleteIn diom_models.CacheDeleteIn,
	o *CacheDeleteOptions,
) (*diom_models.CacheDeleteOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.CacheDeleteIn, diom_models.CacheDeleteOut](
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
