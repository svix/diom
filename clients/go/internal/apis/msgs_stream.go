package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type MsgsStream struct {
	client *coyote_proto.HttpClient
}

func NewMsgsStream(client *coyote_proto.HttpClient) MsgsStream {
	return MsgsStream{client}
}

// Receives messages from a topic using a consumer group.
//
// Each consumer in the group reads from all partitions. Messages are locked by leases for the
// specified duration to prevent duplicate delivery within the same consumer group.
func (msgsStream MsgsStream) Receive(
	ctx context.Context,
	msgStreamReceiveIn coyote_models.MsgStreamReceiveIn,
) (*coyote_models.MsgStreamReceiveOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.MsgStreamReceiveIn, coyote_models.MsgStreamReceiveOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/receive",
		nil,
		nil,
		&msgStreamReceiveIn,
	)
}

// Commits an offset for a consumer group on a specific partition.
//
// The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
// successfully processed offset; future receives will start after it.
func (msgsStream MsgsStream) Commit(
	ctx context.Context,
	msgStreamCommitIn coyote_models.MsgStreamCommitIn,
) (*coyote_models.MsgStreamCommitOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.MsgStreamCommitIn, coyote_models.MsgStreamCommitOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/commit",
		nil,
		nil,
		&msgStreamCommitIn,
	)
}

// Repositions a consumer group's read cursor on a topic.
//
// Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
// partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
// `"latest"` and may be used with or without a partition suffix.
func (msgsStream MsgsStream) Seek(
	ctx context.Context,
	msgStreamSeekIn coyote_models.MsgStreamSeekIn,
) (*coyote_models.MsgStreamSeekOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.MsgStreamSeekIn, coyote_models.MsgStreamSeekOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/seek",
		nil,
		nil,
		&msgStreamSeekIn,
	)
}
