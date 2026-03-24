package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
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
	name string,
	msgNamespaceCreateIn diom_models.MsgNamespaceCreateIn,
) (*diom_models.MsgNamespaceCreateOut, error) {
	body := diom_models.MsgNamespaceCreateIn_{
		Name:        name,
		Retention:   msgNamespaceCreateIn.Retention,
		StorageType: msgNamespaceCreateIn.StorageType,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgNamespaceCreateIn_, diom_models.MsgNamespaceCreateOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/create",
		&body,
	)
}

// Gets a msgs namespace by name.
func (msgsNamespace MsgsNamespace) Get(
	ctx context.Context,
	name string,
	msgNamespaceGetIn diom_models.MsgNamespaceGetIn,
) (*diom_models.MsgNamespaceGetOut, error) {
	body := diom_models.MsgNamespaceGetIn_{
		Name: name,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgNamespaceGetIn_, diom_models.MsgNamespaceGetOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/get",
		&body,
	)
}
