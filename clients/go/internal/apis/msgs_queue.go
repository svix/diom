package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type MsgsQueue struct {
	client *coyote_proto.HttpClient
}

func NewMsgsQueue(client *coyote_proto.HttpClient) MsgsQueue {
	return MsgsQueue{client}
}

// Receives messages from a topic as competing consumers.
//
// Messages are individually leased for the specified duration. Multiple consumers can receive
// different messages from the same topic concurrently. Leased messages are skipped until they
// are acked or their lease expires.
func (msgsQueue MsgsQueue) Receive(
	ctx context.Context,
	msgQueueReceiveIn coyote_models.MsgQueueReceiveIn,
) (*coyote_models.MsgQueueReceiveOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.MsgQueueReceiveIn, coyote_models.MsgQueueReceiveOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/receive",
		nil,
		nil,
		&msgQueueReceiveIn,
	)
}

// Acknowledges messages by their opaque msg_ids.
//
// Acked messages are permanently removed from the queue and will never be re-delivered.
func (msgsQueue MsgsQueue) Ack(
	ctx context.Context,
	msgQueueAckIn coyote_models.MsgQueueAckIn,
) (*coyote_models.MsgQueueAckOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.MsgQueueAckIn, coyote_models.MsgQueueAckOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/ack",
		nil,
		nil,
		&msgQueueAckIn,
	)
}
