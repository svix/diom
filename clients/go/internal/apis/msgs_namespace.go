package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type MsgsNamespace struct {
	client *diom_proto.HttpClient
}

func NewMsgsNamespace(client *diom_proto.HttpClient) MsgsNamespace {
	return MsgsNamespace{client}
}

// Creates or updates a msgs namespace with the given name.
func (msgsNamespace MsgsNamespace) Create(
	ctx context.Context,
	msgNamespaceCreateIn diom_models.MsgNamespaceCreateIn,
) (*diom_models.MsgNamespaceCreateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgNamespaceCreateIn, diom_models.MsgNamespaceCreateOut](
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
	msgNamespaceGetIn diom_models.MsgNamespaceGetIn,
) (*diom_models.MsgNamespaceGetOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgNamespaceGetIn, diom_models.MsgNamespaceGetOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/get",
		nil,
		nil,
		&msgNamespaceGetIn,
	)
}
