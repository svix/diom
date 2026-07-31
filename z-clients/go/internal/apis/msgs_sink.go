package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type MsgsSink struct {
	client *diom_proto.HttpClient
}

func NewMsgsSink(client *diom_proto.HttpClient) MsgsSink {
	return MsgsSink{client}
}

// Create or update a sink for a topic. Overwrites any existing sink with the same id.
func (msgsSink MsgsSink) Configure(
	ctx context.Context,
	sinkConfigureIn diom_models.SinkConfigureIn,
) (*diom_models.SinkConfigureOut, error) {
	return diom_proto.ExecuteRequest[diom_models.SinkConfigureIn, diom_models.SinkConfigureOut](
		ctx,
		msgsSink.client,
		"POST",
		"/api/v1.msgs.sink.configure",
		&sinkConfigureIn,
	)
}

// Delete a sink.
func (msgsSink MsgsSink) Delete(
	ctx context.Context,
	sinkDeleteIn diom_models.SinkDeleteIn,
) (*diom_models.SinkDeleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.SinkDeleteIn, diom_models.SinkDeleteOut](
		ctx,
		msgsSink.client,
		"POST",
		"/api/v1.msgs.sink.delete",
		&sinkDeleteIn,
	)
}

// List sink configurations for a topic.
func (msgsSink MsgsSink) List(
	ctx context.Context,
	topic string,
	sinkListIn diom_models.SinkListIn,
) (*diom_models.ListResponseSinkOut, error) {
	body := diom_models.SinkListIn_{
		Namespace: sinkListIn.Namespace,
		Topic:     topic,
		Limit:     sinkListIn.Limit,
		Iterator:  sinkListIn.Iterator,
	}

	return diom_proto.ExecuteRequest[diom_models.SinkListIn_, diom_models.ListResponseSinkOut](
		ctx,
		msgsSink.client,
		"POST",
		"/api/v1.msgs.sink.list",
		&body,
	)
}
