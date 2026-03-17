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
	name string,
	msgNamespaceCreateIn coyote_models.MsgNamespaceCreateIn,
) (*coyote_models.MsgNamespaceCreateOut, error) {
	body := coyote_models.MsgNamespaceCreateIn_{
		Name:        name,
		Retention:   msgNamespaceCreateIn.Retention,
		StorageType: msgNamespaceCreateIn.StorageType,
	}

	return coyote_proto.ExecuteRequest[coyote_models.MsgNamespaceCreateIn_, coyote_models.MsgNamespaceCreateOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/create",
		nil,
		nil,
		&body,
	)
}

// Gets a msgs namespace by name.
func (msgsNamespace MsgsNamespace) Get(
	ctx context.Context,
	name string,
	msgNamespaceGetIn coyote_models.MsgNamespaceGetIn,
) (*coyote_models.MsgNamespaceGetOut, error) {
	body := coyote_models.MsgNamespaceGetIn_{
		Name: name,
	}

	return coyote_proto.ExecuteRequest[coyote_models.MsgNamespaceGetIn_, coyote_models.MsgNamespaceGetOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1/msgs/namespace/get",
		nil,
		nil,
		&body,
	)
}
