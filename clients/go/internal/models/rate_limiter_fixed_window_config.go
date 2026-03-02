package diom_models

// This file is @generated DO NOT EDIT

type RateLimiterFixedWindowConfig struct {
	WindowSize  uint64 `json:"window_size"`  // Window size in seconds
	MaxRequests uint64 `json:"max_requests"` // Maximum number of requests allowed within the window
}
