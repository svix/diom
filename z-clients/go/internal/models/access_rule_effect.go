package coyote_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type AccessRuleEffect string

const (
	ACCESSRULEEFFECT_ALLOW AccessRuleEffect = "allow"
	ACCESSRULEEFFECT_DENY  AccessRuleEffect = "deny"
)

var allowedAccessRuleEffect = []AccessRuleEffect{
	"allow",
	"deny",
}

func (v *AccessRuleEffect) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := AccessRuleEffect(value)
	if slices.Contains(allowedAccessRuleEffect, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid AccessRuleEffect", value)

}

var AccessRuleEffectFromString = map[string]AccessRuleEffect{
	"allow": ACCESSRULEEFFECT_ALLOW,
	"deny":  ACCESSRULEEFFECT_DENY,
}
