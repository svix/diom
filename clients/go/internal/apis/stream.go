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

// Upserts a new Stream with the given name.
func (stream Stream) Create(
	ctx context.Context,
	createStreamIn diom_models.CreateStreamIn,
) (*diom_models.CreateStreamOut, error) {
	return diom_proto.ExecuteRequest[diom_models.CreateStreamIn, diom_models.CreateStreamOut](
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
	getStreamIn diom_models.GetStreamIn,
) (*diom_models.GetStreamOut, error) {
	return diom_proto.ExecuteRequest[diom_models.GetStreamIn, diom_models.GetStreamOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/get-namespace",
		nil,
		nil,
		&getStreamIn,
	)
}

// Appends messages to the stream.
func (stream Stream) Append(
	ctx context.Context,
	appendToStreamIn diom_models.AppendToStreamIn,
) (*diom_models.AppendToStreamOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AppendToStreamIn, diom_models.AppendToStreamOut](
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
	fetchFromStreamIn diom_models.FetchFromStreamIn,
) (*diom_models.FetchFromStreamOut, error) {
	return diom_proto.ExecuteRequest[diom_models.FetchFromStreamIn, diom_models.FetchFromStreamOut](
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
	fetchFromStreamIn diom_models.FetchFromStreamIn,
) (*diom_models.FetchFromStreamOut, error) {
	return diom_proto.ExecuteRequest[diom_models.FetchFromStreamIn, diom_models.FetchFromStreamOut](
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
	ackMsgRangeIn diom_models.AckMsgRangeIn,
) (*diom_models.AckMsgRangeOut, error) {
	return diom_proto.ExecuteRequest[diom_models.AckMsgRangeIn, diom_models.AckMsgRangeOut](
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
	ack diom_models.Ack,
) (*diom_models.AckOut, error) {
	return diom_proto.ExecuteRequest[diom_models.Ack, diom_models.AckOut](
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
	dlqIn diom_models.DlqIn,
) (*diom_models.DlqOut, error) {
	return diom_proto.ExecuteRequest[diom_models.DlqIn, diom_models.DlqOut](
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
	redriveIn diom_models.RedriveIn,
) (*diom_models.RedriveOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RedriveIn, diom_models.RedriveOut](
		ctx,
		stream.client,
		"POST",
		"/api/v1/stream/redrive-dlq",
		nil,
		nil,
		&redriveIn,
	)
}
