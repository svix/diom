package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type SeekPosition string

const (
	SEEKPOSITION_EARLIEST SeekPosition = "earliest"
	SEEKPOSITION_LATEST   SeekPosition = "latest"
)

var allowedSeekPosition = []SeekPosition{
	"earliest",
	"latest",
}

func (v *SeekPosition) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := SeekPosition(value)
	if slices.Contains(allowedSeekPosition, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid SeekPosition", value)

}

var SeekPositionFromString = map[string]SeekPosition{
	"earliest": SEEKPOSITION_EARLIEST,
	"latest":   SEEKPOSITION_LATEST,
}
