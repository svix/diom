package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type MsgsNamespace struct {
	client *coyote_proto.HttpClient
}

func NewMsgsNamespace(client *coyote_proto.HttpClient) MsgsNamespace {
	return MsgsNamespace{client}
}

// Creates or updates a msgs namespace with the given name.
func (msgsNamespace MsgsNamespace) Create(
	ctx context.Context,
	createNamespaceIn coyote_models.CreateNamespaceIn,
) (*coyote_models.CreateNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CreateNamespaceIn, coyote_models.CreateNamespaceOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/create",
		nil,
		nil,
		&createNamespaceIn,
	)
}

// Gets a msgs namespace by name.
func (msgsNamespace MsgsNamespace) Get(
	ctx context.Context,
	getNamespaceIn coyote_models.GetNamespaceIn,
) (*coyote_models.GetNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.GetNamespaceIn, coyote_models.GetNamespaceOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/get",
		nil,
		nil,
		&getNamespaceIn,
	)
}
