package diom

import (
	diom_proto "diom.com/go/diom/internal/proto"
	diom_types "diom.com/go/diom/internal/types"
)

type (
	ConnectionError = diom_proto.ConnectionError
	InvalidInput    = diom_proto.InvalidInput
	OperationError  = diom_proto.OperationError
	ServerError     = diom_proto.ServerError
	OtherError      = diom_proto.OtherError
	DurationMs      = diom_types.DurationMs
	Timestamp       = diom_types.Timestamp
)
