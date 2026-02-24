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

// Upserts a new Stream with the given name.
func (stream Stream) Create(
	ctx context.Context,
	createStreamIn coyote_models.CreateStreamIn,
) (*coyote_models.CreateStreamOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.CreateStreamIn, coyote_models.CreateStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/create",
		nil,
		nil,
		&createStreamIn,
	)
}

// Get stream with given name.
func (stream Stream) Get(
	ctx context.Context,
	getStreamIn coyote_models.GetStreamIn,
) (*coyote_models.GetStreamOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.GetStreamIn, coyote_models.GetStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/get-group",
		nil,
		nil,
		&getStreamIn,
	)
}

// Appends messages to the stream.
func (stream Stream) Append(
	ctx context.Context,
	appendToStreamIn coyote_models.AppendToStreamIn,
) (*coyote_models.AppendToStreamOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AppendToStreamIn, coyote_models.AppendToStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/append",
		nil,
		nil,
		&appendToStreamIn,
	)
}

// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
//
// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
// until the visibility timeout expires, or the messages are acked.
func (stream Stream) Fetch(
	ctx context.Context,
	fetchFromStreamIn coyote_models.FetchFromStreamIn,
) (*coyote_models.FetchFromStreamOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.FetchFromStreamIn, coyote_models.FetchFromStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/fetch",
		nil,
		nil,
		&fetchFromStreamIn,
	)
}

// Fetches messages from the stream, locking over the consumer group.
//
// This call prevents other consumers within the same consumer group from reading from the stream
// until either the visibility timeout expires, or the last message in the batch is acknowledged.
func (stream Stream) FetchLocking(
	ctx context.Context,
	fetchFromStreamIn coyote_models.FetchFromStreamIn,
) (*coyote_models.FetchFromStreamOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.FetchFromStreamIn, coyote_models.FetchFromStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/fetch-locking",
		nil,
		nil,
		&fetchFromStreamIn,
	)
}

// Acks the messages for the consumer group, allowing more messages to be consumed.
func (stream Stream) AckRange(
	ctx context.Context,
	ackMsgRangeIn coyote_models.AckMsgRangeIn,
) (*coyote_models.AckMsgRangeOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.AckMsgRangeIn, coyote_models.AckMsgRangeOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/ack-range",
		nil,
		nil,
		&ackMsgRangeIn,
	)
}

// Acks a single message.
func (stream Stream) Ack(
	ctx context.Context,
	ack coyote_models.Ack,
) (*coyote_models.AckOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.Ack, coyote_models.AckOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/ack",
		nil,
		nil,
		&ack,
	)
}

// Moves a message to the dead letter queue.
func (stream Stream) Dlq(
	ctx context.Context,
	dlqIn coyote_models.DlqIn,
) (*coyote_models.DlqOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.DlqIn, coyote_models.DlqOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/dlq",
		nil,
		nil,
		&dlqIn,
	)
}

// Redrives messages from the dead letter queue back to the stream.
func (stream Stream) Redrive(
	ctx context.Context,
	redriveIn coyote_models.RedriveIn,
) (*coyote_models.RedriveOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RedriveIn, coyote_models.RedriveOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/redrive-dlq",
		nil,
		nil,
		&redriveIn,
	)
}
