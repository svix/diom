// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum ServerState {
    LEADER("Leader"),
    FOLLOWER("Follower"),
    LEARNER("Learner"),
    CANDIDATE("Candidate"),
    SHUTDOWN("Shutdown"),
    UNKNOWN("Unknown");
    private final String value;

    ServerState(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}