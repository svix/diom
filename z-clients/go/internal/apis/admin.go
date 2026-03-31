package diom_apis

// This file is @generated DO NOT EDIT

import (
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type Admin struct {
	client *diom_proto.HttpClient
}

func NewAdmin(client *diom_proto.HttpClient) Admin {
	return Admin{client}
}

func (admin Admin) AuthRole() AdminAuthRole {
	return NewAdminAuthRole(admin.client)
}
func (admin Admin) AuthToken() AdminAuthToken {
	return NewAdminAuthToken(admin.client)
}
func (admin Admin) Cluster() AdminCluster {
	return NewAdminCluster(admin.client)
}
