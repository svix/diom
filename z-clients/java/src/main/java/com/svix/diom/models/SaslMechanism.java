// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum SaslMechanism {
    PLAIN("plain"),
    SCRAM_SHA256("scram-sha256"),
    SCRAM_SHA512("scram-sha512");
    private final String value;

    SaslMechanism(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}