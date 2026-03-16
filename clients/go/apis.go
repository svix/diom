package coyote

// This file is @generated DO NOT EDIT

import (
	coyote_apis "github.com/svix/coyote/clients/go/internal/apis"
)

type Admin = coyote_apis.Admin

func (coyote Coyote) Admin() Admin {
	return coyote_apis.NewAdmin(&coyote.inner)
}

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

type Msgs = coyote_apis.Msgs

func (coyote Coyote) Msgs() Msgs {
	return coyote_apis.NewMsgs(&coyote.inner)
}

type RateLimit = coyote_apis.RateLimit

func (coyote Coyote) RateLimit() RateLimit {
	return coyote_apis.NewRateLimit(&coyote.inner)
}
