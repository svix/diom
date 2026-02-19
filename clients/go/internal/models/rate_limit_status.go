package coyote_models

// This file is @generated DO NOT EDIT

import (
	"encoding/json"
	"fmt"
	"slices"
)

type RateLimitStatus string

const (
RATELIMITSTATUS_OK RateLimitStatus = "ok"
RATELIMITSTATUS_BLOCK RateLimitStatus = "block"
)

var allowedRateLimitStatus = []RateLimitStatus{
	"ok",
	"block",
	}


func (v *RateLimitStatus) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := RateLimitStatus(value)
	if slices.Contains(allowedRateLimitStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid RateLimitStatus", value)

}

var RateLimitStatusFromString = map[string]RateLimitStatus{
	"ok" : RATELIMITSTATUS_OK,
	"block" : RATELIMITSTATUS_BLOCK,
	}