package diom_proto

import (
	"fmt"
	"strings"

	"github.com/vmihailenco/msgpack/v5"
)

type ConnectionError struct {
	inner error
}

func (e *ConnectionError) Error() string {
	return "connection error: " + e.inner.Error()
}

func (e *ConnectionError) Unwrap() error {
	return e.inner
}

type InvalidInput struct {
	code     string
	detail   string
	location *string
}

func (e *InvalidInput) Error() string {
	return errorResponse(e.code, e.detail, e.location)
}

func (e *InvalidInput) Code() string {
	return e.code
}

func (e *InvalidInput) Detail() string {
	return e.detail
}

func (e *InvalidInput) Location() *string {
	return e.location
}

type OperationError struct {
	code     string
	detail   string
	location *string
}

func (e *OperationError) Error() string {
	return errorResponse(e.code, e.detail, e.location)
}

func (e *OperationError) Code() string {
	return e.code
}

func (e *OperationError) Detail() string {
	return e.detail
}

type ServerError struct {
	code     string
	detail   string
	location *string
}

func (e *ServerError) Error() string {
	return errorResponse(e.code, e.detail, e.location)
}

func (e *ServerError) Code() string {
	return e.code
}

func (e *ServerError) Detail() string {
	return e.detail
}

type OtherError struct {
	inner error
}

func (e *OtherError) Error() string {
	return "internal error: " + e.inner.Error()
}

func (e *OtherError) Unwrap() error {
	return e.inner
}

func errorResponse(code string, detail string, location *string) string {
	result := fmt.Sprintf("code=%s", code)
	if location != nil {
		result += fmt.Sprintf(" location=%s", *location)
	}
	result += fmt.Sprintf(" detail=\"%s\"", strings.ReplaceAll(detail, "\"", "\\\""))

	return result
}

func newConnectionError(e error) error {
	return &ConnectionError{inner: e}
}

func newOtherError(e error) error {
	return &OtherError{inner: e}
}

func newResponseError(http_body []byte) error {
	var b errorBody
	if err := msgpack.Unmarshal(http_body, &b); err != nil {
		return newOtherError(err)
	}

	switch b.Type_ {
	case "invalid-input":
		return &InvalidInput{code: b.Code, detail: b.Detail, location: b.Location}
	case "operation-error":
		return &OperationError{code: b.Code, detail: b.Detail, location: b.Location}
	case "server-error":
		return &ServerError{code: b.Code, detail: b.Detail, location: b.Location}
	default:
		return newOtherError(fmt.Errorf("invalid error type `%s`", b.Type_))
	}
}

type errorBody struct {
	Type_    string  `msgpack:"type"`
	Code     string  `msgpack:"code"`
	Detail   string  `msgpack:"detail"`
	Location *string `msgpack:"location"`
}
