package diom_models

// This file is @generated DO NOT EDIT

type MetricOut struct {
	Label       string            `msgpack:"label"`       // Label for this series
	Description string            `msgpack:"description"` // Human-readable description of this series
	Attributes  map[string]string `msgpack:"attributes"`  // Key/Value pairs attached to this sequence
	// Most recent data point for this series
	//
	// All points (u64, i64, and f64) are squished into an f64, be careful
	// of inexactness for values above 2**53.
	Value float64 `msgpack:"value"`
	// Type of this metric
	//
	// Histograms are not currently exported through this API, and can
	// only be accessed through OTLP.
	MetricType MetricType `msgpack:"metric_type"`
	Timestamp  uint64     `msgpack:"timestamp"` // Timestamp this metric was collected
	// Optional unit, following UCUM unit conventions if possible
	//
	// See https://ucum.org/ for details
	Unit *string `msgpack:"unit,omitempty"`
}
