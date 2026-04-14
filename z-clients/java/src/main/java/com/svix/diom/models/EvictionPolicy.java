// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum EvictionPolicy {
    NO_EVICTION("no-eviction");
    private final String value;

    EvictionPolicy(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}