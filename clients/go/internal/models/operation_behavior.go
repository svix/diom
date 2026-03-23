package coyote_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type OperationBehavior string

const (
	OPERATIONBEHAVIOR_UPSERT OperationBehavior = "upsert"
	OPERATIONBEHAVIOR_INSERT OperationBehavior = "insert"
	OPERATIONBEHAVIOR_UPDATE OperationBehavior = "update"
)

var allowedOperationBehavior = []OperationBehavior{
	"upsert",
	"insert",
	"update",
}

func (v *OperationBehavior) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := OperationBehavior(value)
	if slices.Contains(allowedOperationBehavior, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid OperationBehavior", value)

}

var OperationBehaviorFromString = map[string]OperationBehavior{
	"upsert": OPERATIONBEHAVIOR_UPSERT,
	"insert": OPERATIONBEHAVIOR_INSERT,
	"update": OPERATIONBEHAVIOR_UPDATE,
}
