package diom_models

// This file is @generated DO NOT EDIT

import (
	"encoding/json"
	"fmt"
)

// When creating an RateLimiterCheckIn, use the appropriate config structure based on the Type:
//   - "fixed_window": Use RateLimiterFixedWindowConfig
//   - "token_bucket": Use RateLimiterTokenBucketConfig
type RateLimiterCheckIn struct {
	Key    string                   `json:"key"`
	Tokens *uint64                  `json:"tokens,omitempty"` // Number of tokens to consume (default: 1)
	Method RateLimiterCheckInMethod `json:"method"`
	Config RateLimiterCheckInConfig `json:"config"`
}

type RateLimiterCheckInMethod string

const (
	RateLimiterCheckInMethodTokenBucket RateLimiterCheckInMethod = "token_bucket"
	RateLimiterCheckInMethodFixedWindow RateLimiterCheckInMethod = "fixed_window"
)

type RateLimiterCheckInConfig interface {
	isRateLimiterCheckInConfig()
}

func (RateLimiterTokenBucketConfig) isRateLimiterCheckInConfig() {}
func (RateLimiterFixedWindowConfig) isRateLimiterCheckInConfig() {}

func (i *RateLimiterCheckIn) UnmarshalJSON(data []byte) error {
	type Alias RateLimiterCheckIn
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

func (i RateLimiterCheckIn) MarshalJSON() ([]byte, error) {
	type Alias RateLimiterCheckIn
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var RateLimiterCheckInMethodFromString = map[string]RateLimiterCheckInMethod{
	"token_bucket": RateLimiterCheckInMethodTokenBucket,
	"fixed_window": RateLimiterCheckInMethodFixedWindow,
}
