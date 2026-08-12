package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

// The connection security protocol, mapped onto librdkafka's `security.protocol`.
type SecurityProtocol string

const (
	SECURITYPROTOCOL_PLAINTEXT      SecurityProtocol = "plaintext"
	SECURITYPROTOCOL_SSL            SecurityProtocol = "ssl"
	SECURITYPROTOCOL_SASL_PLAINTEXT SecurityProtocol = "sasl-plaintext"
	SECURITYPROTOCOL_SASL_SSL       SecurityProtocol = "sasl-ssl"
)

var allowedSecurityProtocol = []SecurityProtocol{
	"plaintext",
	"ssl",
	"sasl-plaintext",
	"sasl-ssl",
}

func (v *SecurityProtocol) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := SecurityProtocol(value)
	if slices.Contains(allowedSecurityProtocol, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid SecurityProtocol", value)

}

var SecurityProtocolFromString = map[string]SecurityProtocol{
	"plaintext":      SECURITYPROTOCOL_PLAINTEXT,
	"ssl":            SECURITYPROTOCOL_SSL,
	"sasl-plaintext": SECURITYPROTOCOL_SASL_PLAINTEXT,
	"sasl-ssl":       SECURITYPROTOCOL_SASL_SSL,
}
