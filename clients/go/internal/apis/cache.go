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

// Cache Set
func (cache *Cache) Set(
	ctx context.Context,
	cacheSetIn diom_models.CacheSetIn,
) (*diom_models.CacheSetOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CacheSetIn, diom_models.CacheSetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/set",
		nil,
		nil,
		nil,
		&cacheSetIn,
	)
}

// Cache Get
func (cache *Cache) Get(
	ctx context.Context,
	cacheGetIn diom_models.CacheGetIn,
) (*diom_models.CacheGetOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CacheGetIn, diom_models.CacheGetOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/get",
		nil,
		nil,
		nil,
		&cacheGetIn,
	)
}

// Get cache group
func (cache *Cache) GetGroup(
	ctx context.Context,
	cacheGetGroupIn diom_models.CacheGetGroupIn,
) (*diom_models.CacheGetGroupOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CacheGetGroupIn, diom_models.CacheGetGroupOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/get-group",
		nil,
		nil,
		nil,
		&cacheGetGroupIn,
	)
}

// Cache Delete
func (cache *Cache) Delete(
	ctx context.Context,
	cacheDeleteIn diom_models.CacheDeleteIn,
) (*diom_models.CacheDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CacheDeleteIn, diom_models.CacheDeleteOut](
		ctx,
		cache.client,
		"POST",
		"/api/v1/cache/delete",
		nil,
		nil,
		nil,
		&cacheDeleteIn,
	)
}
