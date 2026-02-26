// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum RateLimitStatus {
    OK("ok"),
    BLOCK("block");
    private final String value;

    RateLimitStatus(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}