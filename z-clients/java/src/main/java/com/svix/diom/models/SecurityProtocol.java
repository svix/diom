// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum SecurityProtocol {
    PLAINTEXT("plaintext"),
    SSL("ssl"),
    SASL_PLAINTEXT("sasl-plaintext"),
    SASL_SSL("sasl-ssl");
    private final String value;

    SecurityProtocol(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}