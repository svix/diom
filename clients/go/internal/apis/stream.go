package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Stream struct {
	client *diom_proto.HttpClient
}

func NewStream(client *diom_proto.HttpClient) Stream {
	return Stream{client}
}

type StreamCreateOptions struct {
	IdempotencyKey *string
}

type StreamAppendOptions struct {
	IdempotencyKey *string
}

type StreamFetchOptions struct {
	IdempotencyKey *string
}

type StreamFetchLockingOptions struct {
	IdempotencyKey *string
}

type StreamAckRangeOptions struct {
	IdempotencyKey *string
}

type StreamAckOptions struct {
	IdempotencyKey *string
}

type StreamDlqOptions struct {
	IdempotencyKey *string
}

type StreamRedriveOptions struct {
	IdempotencyKey *string
}

// Upserts a new Stream with the given name.
func (stream *Stream) Create(
	ctx context.Context,
	createStreamIn diom_models.CreateStreamIn,
	o *StreamCreateOptions,
) (*diom_models.CreateStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.CreateStreamIn, diom_models.CreateStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/create",
		nil,
		nil,
		headerMap,
		&createStreamIn,
	)
}

// Appends messages to the stream.
func (stream *Stream) Append(
	ctx context.Context,
	appendToStreamIn diom_models.AppendToStreamIn,
	o *StreamAppendOptions,
) (*diom_models.AppendToStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.AppendToStreamIn, diom_models.AppendToStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/append",
		nil,
		nil,
		headerMap,
		&appendToStreamIn,
	)
}

// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
//
// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
// until the visibility timeout expires, or the messages are acked.
func (stream *Stream) Fetch(
	ctx context.Context,
	fetchFromStreamIn diom_models.FetchFromStreamIn,
	o *StreamFetchOptions,
) (*diom_models.FetchFromStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.FetchFromStreamIn, diom_models.FetchFromStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/fetch",
		nil,
		nil,
		headerMap,
		&fetchFromStreamIn,
	)
}

// Fetches messages from the stream, locking over the consumer group.
//
// This call prevents other consumers within the same consumer group from reading from the stream
// until either the visibility timeout expires, or the last message in the batch is acknowledged.
func (stream *Stream) FetchLocking(
	ctx context.Context,
	fetchFromStreamIn diom_models.FetchFromStreamIn,
	o *StreamFetchLockingOptions,
) (*diom_models.FetchFromStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.FetchFromStreamIn, diom_models.FetchFromStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/fetch-locking",
		nil,
		nil,
		headerMap,
		&fetchFromStreamIn,
	)
}

// Acks the messages for the consumer group, allowing more messages to be consumed.
func (stream *Stream) AckRange(
	ctx context.Context,
	ackMsgRangeIn diom_models.AckMsgRangeIn,
	o *StreamAckRangeOptions,
) (*diom_models.AckMsgRangeOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.AckMsgRangeIn, diom_models.AckMsgRangeOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/ack-range",
		nil,
		nil,
		headerMap,
		&ackMsgRangeIn,
	)
}

// Acks a single message.
func (stream *Stream) Ack(
	ctx context.Context,
	ack diom_models.Ack,
	o *StreamAckOptions,
) (*diom_models.AckOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.Ack, diom_models.AckOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/ack",
		nil,
		nil,
		headerMap,
		&ack,
	)
}

// Moves a message to the dead letter queue.
func (stream *Stream) Dlq(
	ctx context.Context,
	dlqIn diom_models.DlqIn,
	o *StreamDlqOptions,
) (*diom_models.DlqOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.DlqIn, diom_models.DlqOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/dlq",
		nil,
		nil,
		headerMap,
		&dlqIn,
	)
}

// Redrives messages from the dead letter queue back to the stream.
func (stream *Stream) Redrive(
	ctx context.Context,
	redriveIn diom_models.RedriveIn,
	o *StreamRedriveOptions,
) (*diom_models.RedriveOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.RedriveIn, diom_models.RedriveOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/redrive-dlq",
		nil,
		nil,
		headerMap,
		&redriveIn,
	)
}
