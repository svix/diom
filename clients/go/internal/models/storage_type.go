package coyote_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type StorageType string

const (
	STORAGETYPE_PERSISTENT StorageType = "Persistent"
	STORAGETYPE_EPHEMERAL  StorageType = "Ephemeral"
)

var allowedStorageType = []StorageType{
	"Persistent",
	"Ephemeral",
}

func (v *StorageType) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := StorageType(value)
	if slices.Contains(allowedStorageType, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid StorageType", value)

}

var StorageTypeFromString = map[string]StorageType{
	"Persistent": STORAGETYPE_PERSISTENT,
	"Ephemeral":  STORAGETYPE_EPHEMERAL,
}
