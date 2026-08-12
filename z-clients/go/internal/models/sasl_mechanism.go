package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

// The SASL mechanism, mapped onto librdkafka's `sasl.mechanism`.
type SaslMechanism string

const (
	SASLMECHANISM_PLAIN        SaslMechanism = "plain"
	SASLMECHANISM_SCRAM_SHA256 SaslMechanism = "scram-sha256"
	SASLMECHANISM_SCRAM_SHA512 SaslMechanism = "scram-sha512"
)

var allowedSaslMechanism = []SaslMechanism{
	"plain",
	"scram-sha256",
	"scram-sha512",
}

func (v *SaslMechanism) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := SaslMechanism(value)
	if slices.Contains(allowedSaslMechanism, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid SaslMechanism", value)

}

var SaslMechanismFromString = map[string]SaslMechanism{
	"plain":        SASLMECHANISM_PLAIN,
	"scram-sha256": SASLMECHANISM_SCRAM_SHA256,
	"scram-sha512": SASLMECHANISM_SCRAM_SHA512,
}
