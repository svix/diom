package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type Cache struct {
	client *diom_proto.HttpClient
}

func NewCache(client *diom_proto.HttpClient) Cache {
	return Cache{client}
}

func (cache Cache) Namespace() CacheNamespace {
	return NewCacheNamespace(cache.client)
}

// Cache Set
func (cache Cache) Set(
	ctx context.Context,
	key string,
	cacheSetIn diom_models.CacheSetIn,
) (*diom_models.CacheSetOut, error) {
	body := diom_models.CacheSetIn_{
		Namespace: cacheSetIn.Namespace,
		Key:       key,
		Value:     cacheSetIn.Value,
		TtlMs:     cacheSetIn.TtlMs,
	}

	return diom_proto.ExecuteRequest[diom_models.CacheSetIn_, diom_models.CacheSetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1.cache.set",
		&body,
	)
}

// Cache Get
func (cache Cache) Get(
	ctx context.Context,
	key string,
	cacheGetIn diom_models.CacheGetIn,
) (*diom_models.CacheGetOut, error) {
	body := diom_models.CacheGetIn_{
		Namespace:   cacheGetIn.Namespace,
		Key:         key,
		Consistency: cacheGetIn.Consistency,
	}

	return diom_proto.ExecuteRequest[diom_models.CacheGetIn_, diom_models.CacheGetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1.cache.get",
		&body,
	)
}

// Cache Delete
func (cache Cache) Delete(
	ctx context.Context,
	key string,
	cacheDeleteIn diom_models.CacheDeleteIn,
) (*diom_models.CacheDeleteOut, error) {
	body := diom_models.CacheDeleteIn_{
		Namespace: cacheDeleteIn.Namespace,
		Key:       key,
	}

	return diom_proto.ExecuteRequest[diom_models.CacheDeleteIn_, diom_models.CacheDeleteOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1.cache.delete",
		&body,
	)
}
