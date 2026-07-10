package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type MsgsSvixPoller struct {
	client *diom_proto.HttpClient
}

func NewMsgsSvixPoller(client *diom_proto.HttpClient) MsgsSvixPoller {
	return MsgsSvixPoller{client}
}

// Create a Svix poller configuration for a topic.
func (msgsSvixPoller MsgsSvixPoller) Create(
	ctx context.Context,
	svixPollerCreateIn diom_models.SvixPollerCreateIn,
) (*diom_models.SvixPollerCreateOut, error) {
	return diom_proto.ExecuteRequest[diom_models.SvixPollerCreateIn, diom_models.SvixPollerCreateOut](
		ctx,
		msgsSvixPoller.client,
		"POST",
		"/api/v1.msgs.svix-poller.create",
		&svixPollerCreateIn,
	)
}

// Delete a Svix poller configuration.
func (msgsSvixPoller MsgsSvixPoller) Delete(
	ctx context.Context,
	svixPollerDeleteIn diom_models.SvixPollerDeleteIn,
) (*diom_models.SvixPollerDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.SvixPollerDeleteIn, diom_models.SvixPollerDeleteOut](
		ctx,
		msgsSvixPoller.client,
		"POST",
		"/api/v1.msgs.svix-poller.delete",
		&svixPollerDeleteIn,
	)
}

// List Svix poller configurations for a topic.
func (msgsSvixPoller MsgsSvixPoller) List(
	ctx context.Context,
	topic string,
	svixPollerListIn diom_models.SvixPollerListIn,
) (*diom_models.ListResponseSvixPollerOut, error) {
	body := diom_models.SvixPollerListIn_{
		Namespace: svixPollerListIn.Namespace,
		Topic:     topic,
		Limit:     svixPollerListIn.Limit,
		Iterator:  svixPollerListIn.Iterator,
	}

	return diom_proto.ExecuteRequest[diom_models.SvixPollerListIn_, diom_models.ListResponseSvixPollerOut](
		ctx,
		msgsSvixPoller.client,
		"POST",
		"/api/v1.msgs.svix-poller.list",
		&body,
	)
}
