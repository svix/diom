package diom_models

// This file is @generated DO NOT EDIT

import (
	"fmt"
	"slices"

	"github.com/vmihailenco/msgpack/v5"
)

type ServerState string

const (
	SERVERSTATE_LEADER    ServerState = "leader"
	SERVERSTATE_FOLLOWER  ServerState = "follower"
	SERVERSTATE_LEARNER   ServerState = "learner"
	SERVERSTATE_CANDIDATE ServerState = "candidate"
	SERVERSTATE_SHUTDOWN  ServerState = "shutdown"
	SERVERSTATE_UNKNOWN   ServerState = "unknown"
)

var allowedServerState = []ServerState{
	"leader",
	"follower",
	"learner",
	"candidate",
	"shutdown",
	"unknown",
}

func (v *ServerState) UnmarshalMsgpack(src []byte) error {
	var value string
	err := msgpack.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := ServerState(value)
	if slices.Contains(allowedServerState, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid ServerState", value)

}

var ServerStateFromString = map[string]ServerState{
	"leader":    SERVERSTATE_LEADER,
	"follower":  SERVERSTATE_FOLLOWER,
	"learner":   SERVERSTATE_LEARNER,
	"candidate": SERVERSTATE_CANDIDATE,
	"shutdown":  SERVERSTATE_SHUTDOWN,
	"unknown":   SERVERSTATE_UNKNOWN,
}
