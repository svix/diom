package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type MsgsQueue struct {
	client *diom_proto.HttpClient
}

func NewMsgsQueue(client *diom_proto.HttpClient) MsgsQueue {
	return MsgsQueue{client}
}

// Receives messages from a topic as competing consumers.
//
// Messages are individually leased for the specified duration. Multiple consumers can receive
// different messages from the same topic concurrently. Leased messages are skipped until they
// are acked or their lease expires.
func (msgsQueue MsgsQueue) Receive(
	ctx context.Context,
	msgQueueReceiveIn diom_models.MsgQueueReceiveIn,
) (*diom_models.MsgQueueReceiveOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgQueueReceiveIn, diom_models.MsgQueueReceiveOut](
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
	msgQueueAckIn diom_models.MsgQueueAckIn,
) (*diom_models.MsgQueueAckOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgQueueAckIn, diom_models.MsgQueueAckOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/ack",
		nil,
		nil,
		&msgQueueAckIn,
	)
}

// Rejects messages, sending them to the dead-letter queue.
//
// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
// move them back to the queue for reprocessing.
func (msgsQueue MsgsQueue) Nack(
	ctx context.Context,
	msgQueueNackIn diom_models.MsgQueueNackIn,
) (*diom_models.MsgQueueNackOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgQueueNackIn, diom_models.MsgQueueNackOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/nack",
		nil,
		nil,
		&msgQueueNackIn,
	)
}

// Moves all dead-letter queue messages back to the main queue for reprocessing.
func (msgsQueue MsgsQueue) RedriveDlq(
	ctx context.Context,
	msgQueueRedriveDlqIn diom_models.MsgQueueRedriveDlqIn,
) (*diom_models.MsgQueueRedriveDlqOut, error) {
	return diom_proto.ExecuteRequest[diom_models.MsgQueueRedriveDlqIn, diom_models.MsgQueueRedriveDlqOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/redrive-dlq",
		nil,
		nil,
		&msgQueueRedriveDlqIn,
	)
}
