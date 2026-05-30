// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum MetricType {
    COUNTER("counter"),
    GAUGE("gauge");
    private final String value;

    MetricType(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}