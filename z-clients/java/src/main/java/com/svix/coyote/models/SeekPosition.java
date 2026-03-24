// this file is @generated
package com.svix.coyote.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum SeekPosition {
    EARLIEST("earliest"),
    LATEST("latest");
    private final String value;

    SeekPosition(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}