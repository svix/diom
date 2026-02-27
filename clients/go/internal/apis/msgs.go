package coyote_apis

// This file is @generated DO NOT EDIT

import (
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Msgs struct {
	Namespace *MsgsNamespace
}

func NewMsgs(client *coyote_proto.HttpClient) Msgs {
	return Msgs{client}
}
