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
	streamReceiveIn coyote_models.StreamReceiveIn,
) (*coyote_models.StreamReceiveOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.StreamReceiveIn, coyote_models.StreamReceiveOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/receive",
		nil,
		nil,
		&streamReceiveIn,
	)
}
