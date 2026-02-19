package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Stream struct {
	client *coyote_proto.HttpClient
}

func NewStream(client *coyote_proto.HttpClient) Stream {
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
	createStreamIn coyote_models.CreateStreamIn,
	o *StreamCreateOptions,
) (*coyote_models.CreateStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.CreateStreamIn, coyote_models.CreateStreamOut](
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
	appendToStreamIn coyote_models.AppendToStreamIn,
	o *StreamAppendOptions,
) (*coyote_models.AppendToStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.AppendToStreamIn, coyote_models.AppendToStreamOut](
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
	fetchFromStreamIn coyote_models.FetchFromStreamIn,
	o *StreamFetchOptions,
) (*coyote_models.FetchFromStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.FetchFromStreamIn, coyote_models.FetchFromStreamOut](
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
	fetchFromStreamIn coyote_models.FetchFromStreamIn,
	o *StreamFetchLockingOptions,
) (*coyote_models.FetchFromStreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.FetchFromStreamIn, coyote_models.FetchFromStreamOut](
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
	ackMsgRangeIn coyote_models.AckMsgRangeIn,
	o *StreamAckRangeOptions,
) (*coyote_models.AckMsgRangeOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.AckMsgRangeIn, coyote_models.AckMsgRangeOut](
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
	ack coyote_models.Ack,
	o *StreamAckOptions,
) (*coyote_models.AckOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.Ack, coyote_models.AckOut](
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
	dlqIn coyote_models.DlqIn,
	o *StreamDlqOptions,
) (*coyote_models.DlqOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.DlqIn, coyote_models.DlqOut](
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
	redriveIn coyote_models.RedriveIn,
	o *StreamRedriveOptions,
) (*coyote_models.RedriveOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.RedriveIn, coyote_models.RedriveOut](
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
