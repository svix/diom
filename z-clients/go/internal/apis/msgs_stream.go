package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type MsgsStream struct {
	client *diom_proto.HttpClient
}

func NewMsgsStream(client *diom_proto.HttpClient) MsgsStream {
	return MsgsStream{client}
}

// Receives messages from a topic using a consumer group.
//
// Each consumer in the group reads from all partitions. Messages are locked by leases for the
// specified duration to prevent duplicate delivery within the same consumer group.
func (msgsStream MsgsStream) Receive(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgStreamReceiveIn diom_models.MsgStreamReceiveIn,
) (*diom_models.MsgStreamReceiveOut, error) {
	body := diom_models.MsgStreamReceiveIn_{
		Namespace:               msgStreamReceiveIn.Namespace,
		Topic:                   topic,
		ConsumerGroup:           consumerGroup,
		BatchSize:               msgStreamReceiveIn.BatchSize,
		LeaseDurationMs:         msgStreamReceiveIn.LeaseDurationMs,
		DefaultStartingPosition: msgStreamReceiveIn.DefaultStartingPosition,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgStreamReceiveIn_, diom_models.MsgStreamReceiveOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/receive",
		&body,
	)
}

// Commits an offset for a consumer group on a specific partition.
//
// The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
// successfully processed offset; future receives will start after it.
func (msgsStream MsgsStream) Commit(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgStreamCommitIn diom_models.MsgStreamCommitIn,
) (*diom_models.MsgStreamCommitOut, error) {
	body := diom_models.MsgStreamCommitIn_{
		Namespace:     msgStreamCommitIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		Offset:        msgStreamCommitIn.Offset,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgStreamCommitIn_, diom_models.MsgStreamCommitOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/commit",
		&body,
	)
}

// Repositions a consumer group's read cursor on a topic.
//
// Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
// partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
// `"latest"` and may be used with or without a partition suffix.
func (msgsStream MsgsStream) Seek(
	ctx context.Context,
	topic string,
	consumerGroup string,
	msgStreamSeekIn diom_models.MsgStreamSeekIn,
) (*diom_models.MsgStreamSeekOut, error) {
	body := diom_models.MsgStreamSeekIn_{
		Namespace:     msgStreamSeekIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		Offset:        msgStreamSeekIn.Offset,
		Position:      msgStreamSeekIn.Position,
	}

	return diom_proto.ExecuteRequest[diom_models.MsgStreamSeekIn_, diom_models.MsgStreamSeekOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/seek",
		&body,
	)
}
