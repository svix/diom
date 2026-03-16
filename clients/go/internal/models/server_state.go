package diom_models

// This file is @generated DO NOT EDIT

import (
	"encoding/json"
	"fmt"
	"slices"
)

type ServerState string

const (
	SERVERSTATE_LEADER    ServerState = "Leader"
	SERVERSTATE_FOLLOWER  ServerState = "Follower"
	SERVERSTATE_LEARNER   ServerState = "Learner"
	SERVERSTATE_CANDIDATE ServerState = "Candidate"
	SERVERSTATE_SHUTDOWN  ServerState = "Shutdown"
	SERVERSTATE_UNKNOWN   ServerState = "Unknown"
)

var allowedServerState = []ServerState{
	"Leader",
	"Follower",
	"Learner",
	"Candidate",
	"Shutdown",
	"Unknown",
}

func (v *ServerState) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
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
	"Leader":    SERVERSTATE_LEADER,
	"Follower":  SERVERSTATE_FOLLOWER,
	"Learner":   SERVERSTATE_LEARNER,
	"Candidate": SERVERSTATE_CANDIDATE,
	"Shutdown":  SERVERSTATE_SHUTDOWN,
	"Unknown":   SERVERSTATE_UNKNOWN,
}
