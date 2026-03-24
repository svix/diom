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
public abstract class IdempotencyStartOutConfig {
    @JsonIgnore
    public String getVariantName() {
        VariantName annotation = this.getClass().getAnnotation(VariantName.class);
        return annotation != null ? annotation.value() : null;
    }

    public abstract JsonNode toJsonNode();

    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("started")
    public static class Started extends IdempotencyStartOutConfig {
        @Override public JsonNode toJsonNode() {
            return Utils.getObjectMapper().createObjectNode();
            }
    }
    @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("locked")
    public static class Locked extends IdempotencyStartOutConfig {
        @Override public JsonNode toJsonNode() {
            return Utils.getObjectMapper().createObjectNode();
            }
    }
    @Getter
        @Setter
        @AllArgsConstructor
        @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("completed")
    public static class Completed extends IdempotencyStartOutConfig {
        private final IdempotencyCompleted completed;
        @Override public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(completed);
            }
    }
    @FunctionalInterface
    private interface TypeFactory {
        IdempotencyStartOutConfig create(JsonNode config);
    }
    private static final Map<String, TypeFactory> TY_M = new HashMap<>();
    private static final ObjectMapper m = Utils.getObjectMapper();
    static {
        TY_M.put("started", c -> new Started());
            TY_M.put("locked", c -> new Locked());
            TY_M.put("completed", c -> new Completed(m.convertValue(c, IdempotencyCompleted.class)));
            }

    public static IdempotencyStartOutConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }

}
