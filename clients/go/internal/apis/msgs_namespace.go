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
	msgNamespaceCreateIn coyote_models.MsgNamespaceCreateIn,
) (*coyote_models.MsgNamespaceCreateOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.MsgNamespaceCreateIn, coyote_models.MsgNamespaceCreateOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/create",
		nil,
		nil,
		&msgNamespaceCreateIn,
	)
}

// Gets a msgs namespace by name.
func (msgsNamespace MsgsNamespace) Get(
	ctx context.Context,
	msgNamespaceGetIn coyote_models.MsgNamespaceGetIn,
) (*coyote_models.MsgNamespaceGetOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.MsgNamespaceGetIn, coyote_models.MsgNamespaceGetOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/get",
		nil,
		nil,
		&msgNamespaceGetIn,
	)
}
