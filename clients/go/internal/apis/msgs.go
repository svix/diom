package diom_apis

// This file is @generated DO NOT EDIT

import (
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Msgs struct {
	Namespace *MsgsNamespace
}

func NewMsgs(client *diom_proto.HttpClient) Msgs {
	return Msgs{client}
}
