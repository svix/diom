package diom

// This file is @generated DO NOT EDIT

import (
	diom_apis "github.com/svix/diom/clients/go/internal/apis"
)

type Cache = diom_apis.Cache

func (diom Diom) Cache() Cache {
	return diom_apis.NewCache(&diom.inner)
}

type Health = diom_apis.Health

func (diom Diom) Health() Health {
	return diom_apis.NewHealth(&diom.inner)
}

type Idempotency = diom_apis.Idempotency

func (diom Diom) Idempotency() Idempotency {
	return diom_apis.NewIdempotency(&diom.inner)
}

type Kv = diom_apis.Kv

func (diom Diom) Kv() Kv {
	return diom_apis.NewKv(&diom.inner)
}

type RateLimiter = diom_apis.RateLimiter

func (diom Diom) RateLimiter() RateLimiter {
	return diom_apis.NewRateLimiter(&diom.inner)
}

type Stream = diom_apis.Stream

func (diom Diom) Stream() Stream {
	return diom_apis.NewStream(&diom.inner)
}
