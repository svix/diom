// this file is @generated
package com.svix.coyote.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum OperationBehavior {
    UPSERT("upsert"),
    INSERT("insert"),
    UPDATE("update");
    private final String value;

    OperationBehavior(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}