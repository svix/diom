package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type CacheNamespace struct {
	client *coyote_proto.HttpClient
}

func NewCacheNamespace(client *coyote_proto.HttpClient) CacheNamespace {
	return CacheNamespace{client}
}

// Create cache namespace
func (cacheNamespace CacheNamespace) Create(
	ctx context.Context,
	cacheCreateNamespaceIn coyote_models.CacheCreateNamespaceIn,
) (*coyote_models.CacheCreateNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CacheCreateNamespaceIn, coyote_models.CacheCreateNamespaceOut](
		ctx,
		cacheNamespace.client,
		"POST",
		"/api/v1/cache/namespace/create",
		&cacheCreateNamespaceIn,
	)
}

// Get cache namespace
func (cacheNamespace CacheNamespace) Get(
	ctx context.Context,
	cacheGetNamespaceIn coyote_models.CacheGetNamespaceIn,
) (*coyote_models.CacheGetNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CacheGetNamespaceIn, coyote_models.CacheGetNamespaceOut](
		ctx,
		cacheNamespace.client,
		"POST",
		"/api/v1/cache/namespace/get",
		&cacheGetNamespaceIn,
	)
}
