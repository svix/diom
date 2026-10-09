package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type MsgsTopic struct {
	client *diom_proto.HttpClient
}

func NewMsgsTopic(client *diom_proto.HttpClient) MsgsTopic {
	return MsgsTopic{client}
}

// Configures the number of partitions for a topic.
//
// Partition count can only be increased, never decreased. The default for a new topic is 1.
func (msgsTopic MsgsTopic) Configure(
	ctx context.Context,
	topic string,
	msgTopicConfigureIn diom_models.MsgTopicConfigureIn,
) (*diom_models.MsgTopicConfigureOut, error) {
	body := diom_models.MsgTopicConfigureIn_{
		Namespace:  msgTopicConfigureIn.Namespace,
		Topic:      topic,
		Partitions: msgTopicConfigureIn.Partitions,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgTopicConfigureIn_, diom_models.MsgTopicConfigureOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1.msgs.topic.configure",
		&body,
	)
}

// List available topics in the given namespace
func (msgsTopic MsgsTopic) List(
	ctx context.Context,
	msgTopicListIn diom_models.MsgTopicListIn,
) (*diom_models.ListResponseMsgTopicListOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgTopicListIn, diom_models.ListResponseMsgTopicListOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1.msgs.topic.list",
		&msgTopicListIn,
	)
}

// Show information about the given topic
func (msgsTopic MsgsTopic) Describe(
	ctx context.Context,
	topic string,
	msgTopicDescribeIn diom_models.MsgTopicDescribeIn,
) (*diom_models.MsgTopicDescribeOut, error) {
	body := diom_models.MsgTopicDescribeIn_{
		Namespace:   msgTopicDescribeIn.Namespace,
		Topic:       topic,
		Consistency: msgTopicDescribeIn.Consistency,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgTopicDescribeIn_, diom_models.MsgTopicDescribeOut](
		ctx,
		msgsTopic.client,
		"POST",
		"/api/v1.msgs.topic.describe",
		&body,
	)
}
