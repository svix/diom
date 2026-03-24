// this file is @generated
package com.svix.coyote.models;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.annotation.JsonFilter;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.coyote.Utils;
import java.util.Map;
import java.util.Set;
import java.util.List;
import java.util.Optional;
import java.util.HashMap;
import java.time.OffsetDateTime;
import java.util.LinkedHashSet;
import java.util.ArrayList;
import java.net.URI;
import java.util.Objects;
import lombok.EqualsAndHashCode;
import lombok.ToString;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class RateLimitResetIn {
    @JsonProperty private String namespace;
    @JsonProperty private String key;
    @JsonProperty private RateLimitTokenBucketConfig config;
    public RateLimitResetIn() {}

    public RateLimitResetIn namespace(String namespace) {
        this.namespace = namespace;
        return this;
    }

    /**
    * Get namespace
    *
     * @return namespace
     */
    @javax.annotation.Nullable
    public String getNamespace() {
        return namespace;
    }

    public void setNamespace(String namespace) {
        this.namespace = namespace;
    }

    public RateLimitResetIn key(String key) {
        this.key = key;
        return this;
    }

    /**
    * Get key
    *
     * @return key
     */
    @javax.annotation.Nonnull
    public String getKey() {
        return key;
    }

    public void setKey(String key) {
        this.key = key;
    }

    public RateLimitResetIn config(RateLimitTokenBucketConfig config) {
        this.config = config;
        return this;
    }

    /**
    * Rate limiter configuration
    *
     * @return config
     */
    @javax.annotation.Nonnull
    public RateLimitTokenBucketConfig getConfig() {
        return config;
    }

    public void setConfig(RateLimitTokenBucketConfig config) {
        this.config = config;
    }

    /**
     * Create an instance of RateLimitResetIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimitResetIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimitResetIn
     */
    public static RateLimitResetIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimitResetIn.class);
    }

    /**
     * Convert an instance of RateLimitResetIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}