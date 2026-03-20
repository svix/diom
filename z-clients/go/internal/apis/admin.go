package coyote_apis

// This file is @generated DO NOT EDIT

import (
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type Admin struct {
	client *coyote_proto.HttpClient
}

func NewAdmin(client *coyote_proto.HttpClient) Admin {
	return Admin{client}
}

func (admin Admin) AuthToken() AdminAuthToken {
	return NewAdminAuthToken(admin.client)
}
func (admin Admin) Cluster() AdminCluster {
	return NewAdminCluster(admin.client)
}
