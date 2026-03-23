package coyote_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	msgpack "github.com/vmihailenco/msgpack/v5"
)

type EvictionPolicy string

const (
	EVICTIONPOLICY_NO_EVICTION         EvictionPolicy = "NoEviction"
	EVICTIONPOLICY_LEAST_RECENTLY_USED EvictionPolicy = "LeastRecentlyUsed"
)

var allowedEvictionPolicy = []EvictionPolicy{
	"NoEviction",
	"LeastRecentlyUsed",
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
	"NoEviction":        EVICTIONPOLICY_NO_EVICTION,
	"LeastRecentlyUsed": EVICTIONPOLICY_LEAST_RECENTLY_USED,
}
