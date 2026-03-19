package diom_models

// This file is @generated DO NOT EDIT

type IdempotencyAbortIn struct {
	Namespace *string `json:"namespace,omitempty"`
}

type IdempotencyAbortIn_ struct {
	Namespace *string `json:"namespace,omitempty"`
	Key       string  `json:"key"`
}
