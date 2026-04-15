package diom_models

// This file is @generated DO NOT EDIT

type MsgNamespaceConfigureIn struct {
	Retention *Retention `msgpack:"retention,omitempty"`
}

type MsgNamespaceConfigureIn_ struct {
	Name      string     `msgpack:"name"`
	Retention *Retention `msgpack:"retention,omitempty"`
}
