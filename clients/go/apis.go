package coyote

// This file is @generated DO NOT EDIT

import (
	coyote_apis "github.com/svix/coyote/clients/go/internal/apis"
)

type Cache = coyote_apis.Cache

func (coyote Coyote) Cache() Cache {
	return coyote_apis.NewCache(&coyote.inner)
}

type Health = coyote_apis.Health

func (coyote Coyote) Health() Health {
	return coyote_apis.NewHealth(&coyote.inner)
}

type Idempotency = coyote_apis.Idempotency

func (coyote Coyote) Idempotency() Idempotency {
	return coyote_apis.NewIdempotency(&coyote.inner)
}

type Kv = coyote_apis.Kv

func (coyote Coyote) Kv() Kv {
	return coyote_apis.NewKv(&coyote.inner)
}

type RateLimiter = coyote_apis.RateLimiter

func (coyote Coyote) RateLimiter() RateLimiter {
	return coyote_apis.NewRateLimiter(&coyote.inner)
}

type Stream = coyote_apis.Stream

func (coyote Coyote) Stream() Stream {
	return coyote_apis.NewStream(&coyote.inner)
}

