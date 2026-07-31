// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum HttpMethod {
    POST("post"),
    PUT("put"),
    PATCH("patch");
    private final String value;

    HttpMethod(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}