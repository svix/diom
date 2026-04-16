package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type CacheNamespace struct {
	client *diom_proto.HttpClient
}

func NewCacheNamespace(client *diom_proto.HttpClient) CacheNamespace {
	return CacheNamespace{client}
}

// Configure cache namespace
func (cacheNamespace CacheNamespace) Configure(
	ctx context.Context,
	cacheConfigureNamespaceIn diom_models.CacheConfigureNamespaceIn,
) (*diom_models.CacheConfigureNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CacheConfigureNamespaceIn, diom_models.CacheConfigureNamespaceOut](
		ctx,
		cacheNamespace.client,
		"POST",
		"/api/v1.cache.namespace.configure",
		&cacheConfigureNamespaceIn,
	)
}

// Get cache namespace
func (cacheNamespace CacheNamespace) Get(
	ctx context.Context,
	cacheGetNamespaceIn diom_models.CacheGetNamespaceIn,
) (*diom_models.CacheGetNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CacheGetNamespaceIn, diom_models.CacheGetNamespaceOut](
		ctx,
		cacheNamespace.client,
		"POST",
		"/api/v1.cache.namespace.get",
		&cacheGetNamespaceIn,
	)
}
