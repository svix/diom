package diom_models

// This file is @generated DO NOT EDIT

import (
	"encoding/json"
	"fmt"
	"slices"
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

func (v *StorageType) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
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
