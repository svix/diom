package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type MsgsNamespace struct {
	client *diom_proto.HttpClient
}

func NewMsgsNamespace(client *diom_proto.HttpClient) MsgsNamespace {
	return MsgsNamespace{client}
}

// Configures a msgs namespace with the given name.
func (msgsNamespace MsgsNamespace) Configure(
	ctx context.Context,
	name string,
	msgNamespaceConfigureIn diom_models.MsgNamespaceConfigureIn,
) (*diom_models.MsgNamespaceConfigureOut, error) {
	body := diom_models.MsgNamespaceConfigureIn_{
		Name:      name,
		Retention: msgNamespaceConfigureIn.Retention,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgNamespaceConfigureIn_, diom_models.MsgNamespaceConfigureOut](
		ctx,
		msgsNamespace.client,
		"POST",
		"/api/v1.msgs.namespace.configure",
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
		"/api/v1.msgs.namespace.get",
		&body,
	)
}
