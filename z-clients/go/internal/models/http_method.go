package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type HttpMethod string

const (
	HTTPMETHOD_POST  HttpMethod = "post"
	HTTPMETHOD_PUT   HttpMethod = "put"
	HTTPMETHOD_PATCH HttpMethod = "patch"
)

var allowedHttpMethod = []HttpMethod{
	"post",
	"put",
	"patch",
}

func (v *HttpMethod) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := HttpMethod(value)
	if slices.Contains(allowedHttpMethod, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid HttpMethod", value)

}

var HttpMethodFromString = map[string]HttpMethod{
	"post":  HTTPMETHOD_POST,
	"put":   HTTPMETHOD_PUT,
	"patch": HTTPMETHOD_PATCH,
}
