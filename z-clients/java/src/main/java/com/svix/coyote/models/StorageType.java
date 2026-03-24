// this file is @generated
package com.svix.coyote.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum StorageType {
    PERSISTENT("Persistent"),
    EPHEMERAL("Ephemeral");
    private final String value;

    StorageType(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}