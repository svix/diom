// This file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.svix.diom.Utils;
import com.svix.diom.VariantName;
import lombok.*;

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
public abstract class SinkConfigConfig {
    @JsonIgnore
    public String getVariantName() {
        VariantName annotation = this.getClass().getAnnotation(VariantName.class);
        return annotation != null ? annotation.value() : null;
    }

    public abstract JsonNode toJsonNode();

    @Getter
        @Setter
        @AllArgsConstructor
        @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("http")
    public static class Http extends SinkConfigConfig {
        private final HttpSinkConfig http;
        @Override public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(http);
            }
    }
    @FunctionalInterface
    private interface TypeFactory {
        SinkConfigConfig create(JsonNode config);
    }
    private static final Map<String, TypeFactory> TY_M = new HashMap<>();
    private static final ObjectMapper m = Utils.getObjectMapper();
    static {
        TY_M.put("http", c -> new Http(m.convertValue(c, HttpSinkConfig.class)));
            }

    public static SinkConfigConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }

}
