package coyote_models

// This file is @generated DO NOT EDIT

import (
	"encoding/json"
	"fmt"
)

// When creating an RateLimiterGetRemainingIn, use the appropriate config structure based on the Type:
//   - "fixed_window": Use RateLimiterFixedWindowConfig
		//   - "token_bucket": Use RateLimiterTokenBucketConfig
		type RateLimiterGetRemainingIn struct {
	Key string `json:"key"`
Method RateLimiterGetRemainingInMethod `json:"method"`
	Config RateLimiterGetRemainingInConfig `json:"config"`
}

type RateLimiterGetRemainingInMethod string

const (
	RateLimiterGetRemainingInMethodTokenBucket RateLimiterGetRemainingInMethod = "token_bucket"
	RateLimiterGetRemainingInMethodFixedWindow RateLimiterGetRemainingInMethod = "fixed_window"
	)


type RateLimiterGetRemainingInConfig interface {
	isRateLimiterGetRemainingInConfig()
}

func (RateLimiterTokenBucketConfig) isRateLimiterGetRemainingInConfig(){}
	func (RateLimiterFixedWindowConfig) isRateLimiterGetRemainingInConfig(){}
	

func (i *RateLimiterGetRemainingIn) UnmarshalJSON(data []byte) error {
	type Alias RateLimiterGetRemainingIn
	aux := struct {
		*Alias
		Config json.RawMessage `json:"config"`
	}{Alias: (*Alias)(i)}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Method {
	case "fixed_window":
			var c RateLimiterFixedWindowConfig
			err = json.Unmarshal(aux.Config, &c)
			i.Config = c
			case "token_bucket":
			var c RateLimiterTokenBucketConfig
			err = json.Unmarshal(aux.Config, &c)
			i.Config = c
			default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Method)
	}
	return err
}

func (i RateLimiterGetRemainingIn) MarshalJSON() ([]byte, error) {
	type Alias RateLimiterGetRemainingIn
	return json.Marshal(&struct {Alias}{Alias: (Alias)(i)})
}

var RateLimiterGetRemainingInMethodFromString = map[string]RateLimiterGetRemainingInMethod{
	"token_bucket" : RateLimiterGetRemainingInMethodTokenBucket,
	"fixed_window" : RateLimiterGetRemainingInMethodFixedWindow,
	}