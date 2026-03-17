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
	topic string,
	consumerGroup string,
	msgQueueReceiveIn diom_models.MsgQueueReceiveIn,
) (*diom_models.MsgQueueReceiveOut, error) {
	body := diom_models.MsgQueueReceiveIn_{
		Topic:               topic,
		ConsumerGroup:       consumerGroup,
		BatchSize:           msgQueueReceiveIn.BatchSize,
		LeaseDurationMillis: msgQueueReceiveIn.LeaseDurationMillis,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgQueueReceiveIn_, diom_models.MsgQueueReceiveOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/receive",
		&body,
	)
}

// Acknowledges messages by their opaque msg_ids.
//
// Acked messages are permanently removed from the queue and will never be re-delivered.
func (msgsQueue MsgsQueue) Ack(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgQueueAckIn diom_models.MsgQueueAckIn,
) (*diom_models.MsgQueueAckOut, error) {
	body := diom_models.MsgQueueAckIn_{
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		MsgIds:        msgQueueAckIn.MsgIds,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgQueueAckIn_, diom_models.MsgQueueAckOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/ack",
		&body,
	)
}

// Configures retry and DLQ behavior for a consumer group on a topic.
//
// `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
// the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
func (msgsQueue MsgsQueue) Configure(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgQueueConfigureIn diom_models.MsgQueueConfigureIn,
) (*diom_models.MsgQueueConfigureOut, error) {
	body := diom_models.MsgQueueConfigureIn_{
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		RetrySchedule: msgQueueConfigureIn.RetrySchedule,
		DlqTopic:      msgQueueConfigureIn.DlqTopic,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgQueueConfigureIn_, diom_models.MsgQueueConfigureOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/configure",
		&body,
	)
}

// Rejects messages, sending them to the dead-letter queue.
//
// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
// move them back to the queue for reprocessing.
func (msgsQueue MsgsQueue) Nack(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgQueueNackIn diom_models.MsgQueueNackIn,
) (*diom_models.MsgQueueNackOut, error) {
	body := diom_models.MsgQueueNackIn_{
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		MsgIds:        msgQueueNackIn.MsgIds,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgQueueNackIn_, diom_models.MsgQueueNackOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/nack",
		&body,
	)
}

// Moves all dead-letter queue messages back to the main queue for reprocessing.
func (msgsQueue MsgsQueue) RedriveDlq(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgQueueRedriveDlqIn diom_models.MsgQueueRedriveDlqIn,
) (*diom_models.MsgQueueRedriveDlqOut, error) {
	body := diom_models.MsgQueueRedriveDlqIn_{
		Topic:         topic,
		ConsumerGroup: consumerGroup,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgQueueRedriveDlqIn_, diom_models.MsgQueueRedriveDlqOut](
		ctx,
		msgsQueue.client,
		"POST",
		"/api/v1/msgs/queue/redrive-dlq",
		&body,
	)
}
