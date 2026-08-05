package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type MsgsFifo struct {
	client *diom_proto.HttpClient
}

func NewMsgsFifo(client *diom_proto.HttpClient) MsgsFifo {
	return MsgsFifo{client}
}

// Receives messages from a topic with strict per-key ordering.
//
// Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
// message for a key, no other consumer receives that key's messages until it is acked (or its
// lease expires). A single call may return several messages of the same key, in order. Keyless
// messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
// split a key across partitions, breaking its order at that boundary.
func (msgsFifo MsgsFifo) Receive(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgFifoReceiveIn diom_models.MsgFifoReceiveIn,
) (*diom_models.MsgFifoReceiveOut, error) {
	body := diom_models.MsgFifoReceiveIn_{
		Namespace:     msgFifoReceiveIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		BatchSize:     msgFifoReceiveIn.BatchSize,
		LeaseDuration: msgFifoReceiveIn.LeaseDuration,
		BatchWait:     msgFifoReceiveIn.BatchWait,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgFifoReceiveIn_, diom_models.MsgFifoReceiveOut](
		ctx,
		msgsFifo.client,
		"POST",
		"/api/v1.msgs.fifo.receive",
		&body,
	)
}

// Acknowledges fifo messages by their opaque msg_ids, releasing each key for its next message.
func (msgsFifo MsgsFifo) Ack(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgFifoAckIn diom_models.MsgFifoAckIn,
) (*diom_models.MsgFifoAckOut, error) {
	body := diom_models.MsgFifoAckIn_{
		Namespace:     msgFifoAckIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		MsgIds:        msgFifoAckIn.MsgIds,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgFifoAckIn_, diom_models.MsgFifoAckOut](
		ctx,
		msgsFifo.client,
		"POST",
		"/api/v1.msgs.fifo.ack",
		&body,
	)
}

// Extends the lease on in-flight fifo messages.
func (msgsFifo MsgsFifo) ExtendLease(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgFifoExtendLeaseIn diom_models.MsgFifoExtendLeaseIn,
) (*diom_models.MsgFifoExtendLeaseOut, error) {
	body := diom_models.MsgFifoExtendLeaseIn_{
		Namespace:     msgFifoExtendLeaseIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		MsgIds:        msgFifoExtendLeaseIn.MsgIds,
		LeaseDuration: msgFifoExtendLeaseIn.LeaseDuration,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgFifoExtendLeaseIn_, diom_models.MsgFifoExtendLeaseOut](
		ctx,
		msgsFifo.client,
		"POST",
		"/api/v1.msgs.fifo.extend-lease",
		&body,
	)
}

// Configures retry and DLQ behavior for a fifo consumer group on a topic.
func (msgsFifo MsgsFifo) Configure(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgFifoConfigureIn diom_models.MsgFifoConfigureIn,
) (*diom_models.MsgFifoConfigureOut, error) {
	body := diom_models.MsgFifoConfigureIn_{
		Namespace:     msgFifoConfigureIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		RetrySchedule: msgFifoConfigureIn.RetrySchedule,
		DlqTopic:      msgFifoConfigureIn.DlqTopic,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgFifoConfigureIn_, diom_models.MsgFifoConfigureOut](
		ctx,
		msgsFifo.client,
		"POST",
		"/api/v1.msgs.fifo.configure",
		&body,
	)
}

// Rejects fifo messages, retrying per the configured schedule then sending them to the DLQ.
func (msgsFifo MsgsFifo) Nack(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgFifoNackIn diom_models.MsgFifoNackIn,
) (*diom_models.MsgFifoNackOut, error) {
	body := diom_models.MsgFifoNackIn_{
		Namespace:     msgFifoNackIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		MsgIds:        msgFifoNackIn.MsgIds,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgFifoNackIn_, diom_models.MsgFifoNackOut](
		ctx,
		msgsFifo.client,
		"POST",
		"/api/v1.msgs.fifo.nack",
		&body,
	)
}

// Moves all dead-letter queue messages for a fifo consumer group back for reprocessing.
func (msgsFifo MsgsFifo) RedriveDlq(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgFifoRedriveDlqIn diom_models.MsgFifoRedriveDlqIn,
) (*diom_models.MsgFifoRedriveDlqOut, error) {
	body := diom_models.MsgFifoRedriveDlqIn_{
		Namespace:     msgFifoRedriveDlqIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgFifoRedriveDlqIn_, diom_models.MsgFifoRedriveDlqOut](
		ctx,
		msgsFifo.client,
		"POST",
		"/api/v1.msgs.fifo.redrive-dlq",
		&body,
	)
}
