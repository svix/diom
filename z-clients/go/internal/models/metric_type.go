package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type MetricType string

const (
	METRICTYPE_COUNTER MetricType = "counter"
	METRICTYPE_GAUGE   MetricType = "gauge"
)

var allowedMetricType = []MetricType{
	"counter",
	"gauge",
}

func (v *MetricType) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := MetricType(value)
	if slices.Contains(allowedMetricType, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid MetricType", value)

}

var MetricTypeFromString = map[string]MetricType{
	"counter": METRICTYPE_COUNTER,
	"gauge":   METRICTYPE_GAUGE,
}
