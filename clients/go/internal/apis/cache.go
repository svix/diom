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

func (cache Cache) Namespace() CacheNamespace {
	return NewCacheNamespace(cache.client)
}

// Cache Set
func (cache Cache) Set(
	ctx context.Context,
	cacheSetIn coyote_models.CacheSetIn,
) (*coyote_models.CacheSetOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CacheSetIn, coyote_models.CacheSetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/set",
		nil,
		nil,
		&cacheSetIn,
	)
}

// Cache Get
func (cache Cache) Get(
	ctx context.Context,
	cacheGetIn coyote_models.CacheGetIn,
) (*coyote_models.CacheGetOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CacheGetIn, coyote_models.CacheGetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/get",
		nil,
		nil,
		&cacheGetIn,
	)
}

// Cache Delete
func (cache Cache) Delete(
	ctx context.Context,
	cacheDeleteIn coyote_models.CacheDeleteIn,
) (*coyote_models.CacheDeleteOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CacheDeleteIn, coyote_models.CacheDeleteOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/delete",
		nil,
		nil,
		&cacheDeleteIn,
	)
}
