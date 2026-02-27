package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
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
	streamReceiveIn diom_models.StreamReceiveIn,
) (*diom_models.StreamReceiveOut, error) {
	return diom_proto.ExecuteRequest[diom_models.StreamReceiveIn, diom_models.StreamReceiveOut](
		ctx,
		msgsStream.client,
		"POST",
		"/api/v1/msgs/stream/receive",
		nil,
		nil,
		&streamReceiveIn,
	)
}
