package coyote_models

// This file is @generated DO NOT EDIT

import (
	"encoding/json"
	"fmt"
	"slices"
)

// Consistency level for reads.
//
// Strong consistency (also known as linearizability) guarantees that a read will see all previous
// writes. Weak consistency allows stale reads, but can save one or more round trip to the leader.
type Consistency string

const (
	CONSISTENCY_STRONG Consistency = "strong"
	CONSISTENCY_WEAK   Consistency = "weak"
)

var allowedConsistency = []Consistency{
	"strong",
	"weak",
}

func (v *Consistency) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := Consistency(value)
	if slices.Contains(allowedConsistency, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid Consistency", value)

}

var ConsistencyFromString = map[string]Consistency{
	"strong": CONSISTENCY_STRONG,
	"weak":   CONSISTENCY_WEAK,
}
