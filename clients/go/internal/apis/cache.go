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
	key string,
	cacheSetIn coyote_models.CacheSetIn,
) (*coyote_models.CacheSetOut, error) {
	body := coyote_models.CacheSetIn_{
		Key:   key,
		Value: cacheSetIn.Value,
		Ttl:   cacheSetIn.Ttl,
	}

	return coyote_proto.ExecuteRequest[coyote_models.CacheSetIn_, coyote_models.CacheSetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/set",
		&body,
	)
}

// Cache Get
func (cache Cache) Get(
	ctx context.Context,
	key string,
	cacheGetIn coyote_models.CacheGetIn,
) (*coyote_models.CacheGetOut, error) {
	body := coyote_models.CacheGetIn_{
		Key:         key,
		Consistency: cacheGetIn.Consistency,
	}

	return coyote_proto.ExecuteRequest[coyote_models.CacheGetIn_, coyote_models.CacheGetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/get",
		&body,
	)
}

// Cache Delete
func (cache Cache) Delete(
	ctx context.Context,
	key string,
	cacheDeleteIn coyote_models.CacheDeleteIn,
) (*coyote_models.CacheDeleteOut, error) {
	body := coyote_models.CacheDeleteIn_{
		Key: key,
	}

	return coyote_proto.ExecuteRequest[coyote_models.CacheDeleteIn_, coyote_models.CacheDeleteOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/delete",
		&body,
	)
}
