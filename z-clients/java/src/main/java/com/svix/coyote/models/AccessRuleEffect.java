// this file is @generated
package com.svix.coyote.models;

import com.fasterxml.jackson.annotation.JsonValue;

public enum AccessRuleEffect {
    ALLOW("allow"),
    DENY("deny");
    private final String value;

    AccessRuleEffect(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }
}