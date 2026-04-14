package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type EvictionPolicy string

const (
	EVICTIONPOLICY_NO_EVICTION EvictionPolicy = "no-eviction"
)

var allowedEvictionPolicy = []EvictionPolicy{
	"no-eviction",
}

func (v *EvictionPolicy) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := EvictionPolicy(value)
	if slices.Contains(allowedEvictionPolicy, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid EvictionPolicy", value)

}

var EvictionPolicyFromString = map[string]EvictionPolicy{
	"no-eviction": EVICTIONPOLICY_NO_EVICTION,
}
