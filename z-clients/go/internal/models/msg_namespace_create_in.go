package diom_models

// This file is @generated DO NOT EDIT

type MsgNamespaceCreateIn struct {
	Retention *Retention `msgpack:"retention,omitempty"`
}

type MsgNamespaceCreateIn_ struct {
	Name      string     `msgpack:"name"`
	Retention *Retention `msgpack:"retention,omitempty"`
}
