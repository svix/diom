package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type CacheNamespace struct {
	client *diom_proto.HttpClient
}

func NewCacheNamespace(client *diom_proto.HttpClient) CacheNamespace {
	return CacheNamespace{client}
}

// Create cache namespace
func (cacheNamespace CacheNamespace) Create(
	ctx context.Context,
	cacheCreateNamespaceIn diom_models.CacheCreateNamespaceIn,
) (*diom_models.CacheCreateNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CacheCreateNamespaceIn, diom_models.CacheCreateNamespaceOut](
		ctx,
		cacheNamespace.client,
		"POST",
		"/api/v1/cache/namespace/create",
		nil,
		nil,
		&cacheCreateNamespaceIn,
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
		"/api/v1/cache/namespace/get",
		nil,
		nil,
		&cacheGetNamespaceIn,
	)
}
