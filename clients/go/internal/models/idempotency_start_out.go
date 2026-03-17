package coyote_models

// This file is @generated DO NOT EDIT

import (
	"encoding/json"
	"fmt"
)

// When creating an IdempotencyStartOut, use the appropriate data structure based on the Type:
//
// - "started","locked": No data needed (nil or just ignore the data field)
// - "completed": Use IdempotencyCompleted
type IdempotencyStartOut struct {
	Status IdempotencyStartOutStatus `json:"status"`
	Data   IdempotencyStartOutData   `json:"data"`
}

type IdempotencyStartOutStatus string

const (
	IdempotencyStartOutStatusStarted   IdempotencyStartOutStatus = "started"
	IdempotencyStartOutStatusLocked    IdempotencyStartOutStatus = "locked"
	IdempotencyStartOutStatusCompleted IdempotencyStartOutStatus = "completed"
)

type IdempotencyStartOutData interface {
	isIdempotencyStartOutData()
}

type idempotencyStartOutEmpty struct{}

func (idempotencyStartOutEmpty) isIdempotencyStartOutData() {}
func (IdempotencyCompleted) isIdempotencyStartOutData()     {}

func (i *IdempotencyStartOut) UnmarshalJSON(data []byte) error {
	type Alias IdempotencyStartOut
	aux := struct {
		*Alias
		Data json.RawMessage `json:"data"`
	}{Alias: (*Alias)(i)}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Status {
	case "started", "locked":
	case "completed":
		var c IdempotencyCompleted
		err = json.Unmarshal(aux.Data, &c)
		i.Data = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Status)
	}
	return err
}

var IdempotencyStartOutStatusWithNoData = map[string]bool{
	"started": true,
	"locked":  true,
}

func (i IdempotencyStartOut) MarshalJSON() ([]byte, error) {
	type Alias IdempotencyStartOut
	if _, found := IdempotencyStartOutStatusWithNoData[string(i.Status)]; found {
		i.Data = idempotencyStartOutEmpty{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var IdempotencyStartOutStatusFromString = map[string]IdempotencyStartOutStatus{
	"started":   IdempotencyStartOutStatusStarted,
	"locked":    IdempotencyStartOutStatusLocked,
	"completed": IdempotencyStartOutStatusCompleted,
}
