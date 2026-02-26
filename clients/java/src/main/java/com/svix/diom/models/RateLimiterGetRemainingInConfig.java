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
public abstract class RateLimiterGetRemainingInConfig {
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
    @VariantName("token_bucket")
    public static class TokenBucket extends RateLimiterGetRemainingInConfig {
        private final RateLimiterTokenBucketConfig tokenBucket;
        @Override public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(tokenBucket);
            }
    }
    @Getter
        @Setter
        @AllArgsConstructor
        @ToString
    @EqualsAndHashCode(callSuper = false)
    @VariantName("fixed_window")
    public static class FixedWindow extends RateLimiterGetRemainingInConfig {
        private final RateLimiterFixedWindowConfig fixedWindow;
        @Override public JsonNode toJsonNode() {
            return Utils.getObjectMapper().valueToTree(fixedWindow);
            }
    }
    @FunctionalInterface
    private interface TypeFactory {
        RateLimiterGetRemainingInConfig create(JsonNode config);
    }
    private static final Map<String, TypeFactory> TY_M = new HashMap<>();
    private static final ObjectMapper m = Utils.getObjectMapper();
    static {
        TY_M.put("token_bucket", c -> new TokenBucket(m.convertValue(c, RateLimiterTokenBucketConfig.class)));
            TY_M.put("fixed_window", c -> new FixedWindow(m.convertValue(c, RateLimiterFixedWindowConfig.class)));
            }

    public static RateLimiterGetRemainingInConfig fromTypeAndConfig(String type, JsonNode config) {
        TypeFactory factory = TY_M.get(type);
        if (factory == null) {
            throw new IllegalArgumentException("Unknown type: " + type);
        }
        return factory.create(config);
    }

}
