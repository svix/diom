package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
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
	topic string,
	consumerGroup string,
	msgStreamReceiveIn coyote_models.MsgStreamReceiveIn,
) (*coyote_models.MsgStreamReceiveOut, error) {
	body := coyote_models.MsgStreamReceiveIn_{
		Namespace:               msgStreamReceiveIn.Namespace,
		Topic:                   topic,
		ConsumerGroup:           consumerGroup,
		BatchSize:               msgStreamReceiveIn.BatchSize,
		LeaseDurationMs:         msgStreamReceiveIn.LeaseDurationMs,
		DefaultStartingPosition: msgStreamReceiveIn.DefaultStartingPosition,
		BatchWaitMs:             msgStreamReceiveIn.BatchWaitMs,
	}

	return coyote_proto.ExecuteRequest[coyote_models.MsgStreamReceiveIn_, coyote_models.MsgStreamReceiveOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1.msgs.stream.receive",
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
	msgStreamCommitIn coyote_models.MsgStreamCommitIn,
) (*coyote_models.MsgStreamCommitOut, error) {
	body := coyote_models.MsgStreamCommitIn_{
		Namespace:     msgStreamCommitIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		Offset:        msgStreamCommitIn.Offset,
	}

	return coyote_proto.ExecuteRequest[coyote_models.MsgStreamCommitIn_, coyote_models.MsgStreamCommitOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1.msgs.stream.commit",
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
	msgStreamSeekIn coyote_models.MsgStreamSeekIn,
) (*coyote_models.MsgStreamSeekOut, error) {
	body := coyote_models.MsgStreamSeekIn_{
		Namespace:     msgStreamSeekIn.Namespace,
		Topic:         topic,
		ConsumerGroup: consumerGroup,
		Offset:        msgStreamSeekIn.Offset,
		Position:      msgStreamSeekIn.Position,
	}

	return coyote_proto.ExecuteRequest[coyote_models.MsgStreamSeekIn_, coyote_models.MsgStreamSeekOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1.msgs.stream.seek",
		&body,
	)
}
