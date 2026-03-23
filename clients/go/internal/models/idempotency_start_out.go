package coyote_models

// This file is @generated DO NOT EDIT

import (
	"fmt"

	msgpack "github.com/vmihailenco/msgpack/v5"
)

// When creating an IdempotencyStartOut, use the appropriate data structure based on the Type:
//
// - "started","locked": No data needed (nil or just ignore the data field)
// - "completed": Use IdempotencyCompleted
type IdempotencyStartOut struct {
	Status IdempotencyStartOutStatus `msgpack:"status"`
	Data   IdempotencyStartOutData   `msgpack:"data"`
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

func (i *IdempotencyStartOut) UnmarshalMsgpack(data []byte) error {
	type Alias IdempotencyStartOut
	aux := struct {
		*Alias
		Data msgpack.RawMessage `msgpack:"data"`
	}{Alias: (*Alias)(i)}

	if err := msgpack.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Status {
	case "started", "locked":
	case "completed":
		var c IdempotencyCompleted
		err = msgpack.Unmarshal(aux.Data, &c)
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

func (i IdempotencyStartOut) MarshalMsgpack() ([]byte, error) {
	type Alias IdempotencyStartOut
	if _, found := IdempotencyStartOutStatusWithNoData[string(i.Status)]; found {
		i.Data = idempotencyStartOutEmpty{}
	}
	return msgpack.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var IdempotencyStartOutStatusFromString = map[string]IdempotencyStartOutStatus{
	"started":   IdempotencyStartOutStatusStarted,
	"locked":    IdempotencyStartOutStatusLocked,
	"completed": IdempotencyStartOutStatusCompleted,
}
