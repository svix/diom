// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum ServerState {
    LEADER("leader"),
    FOLLOWER("follower"),
    LEARNER("learner"),
    CANDIDATE("candidate"),
    SHUTDOWN("shutdown"),
    UNKNOWN("unknown");
    private final String value;

    ServerState(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}