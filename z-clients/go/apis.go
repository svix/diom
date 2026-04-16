package diom

// This file is @generated DO NOT EDIT

import (
	diom_apis "diom.svix.com/go/diom/internal/apis"
)

type Admin = diom_apis.Admin

func (diom Diom) Admin() Admin {
	return diom_apis.NewAdmin(&diom.inner)
}

type Cache = diom_apis.Cache

func (diom Diom) Cache() Cache {
	return diom_apis.NewCache(&diom.inner)
}

type ClusterAdmin = diom_apis.ClusterAdmin

func (diom Diom) ClusterAdmin() ClusterAdmin {
	return diom_apis.NewClusterAdmin(&diom.inner)
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

type Msgs = diom_apis.Msgs

func (diom Diom) Msgs() Msgs {
	return diom_apis.NewMsgs(&diom.inner)
}

type RateLimit = diom_apis.RateLimit

func (diom Diom) RateLimit() RateLimit {
	return diom_apis.NewRateLimit(&diom.inner)
}
