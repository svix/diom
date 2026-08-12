package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"

	"github.com/vmihailenco/msgpack/v5"
)

// When creating an SinkConfig, use the appropriate data structure based on the Type:
//
// - "http": Use HttpSinkConfig
// - "kafka": Use KafkaSinkConfig
// - "svix": Use SvixSinkConfig
type SinkConfig struct {
	Type SinkConfigType `msgpack:"type"`
	Data SinkConfigData `msgpack:"data"`
}

type SinkConfigType string

const (
	SinkConfigTypeHttp  SinkConfigType = "http"
	SinkConfigTypeSvix  SinkConfigType = "svix"
	SinkConfigTypeKafka SinkConfigType = "kafka"
)

type SinkConfigData interface {
	isSinkConfigData()
}

func (HttpSinkConfig) isSinkConfigData()  {}
func (SvixSinkConfig) isSinkConfigData()  {}
func (KafkaSinkConfig) isSinkConfigData() {}

func (i *SinkConfig) UnmarshalMsgpack(data []byte) error {
	type Alias SinkConfig
	aux := struct {
		*Alias
		Data msgpack.RawMessage `msgpack:"data"`
	}{Alias: (*Alias)(i)}

	if err := msgpack.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Type {
	case "http":
		var c HttpSinkConfig
		err = msgpack.Unmarshal(aux.Data, &c)
		i.Data = c
	case "kafka":
		var c KafkaSinkConfig
		err = msgpack.Unmarshal(aux.Data, &c)
		i.Data = c
	case "svix":
		var c SvixSinkConfig
		err = msgpack.Unmarshal(aux.Data, &c)
		i.Data = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Type)
	}
	return err
}

func (i SinkConfig) MarshalMsgpack() ([]byte, error) {
	type Alias SinkConfig
	return msgpack.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var SinkConfigTypeFromString = map[string]SinkConfigType{
	"http":  SinkConfigTypeHttp,
	"svix":  SinkConfigTypeSvix,
	"kafka": SinkConfigTypeKafka,
}
