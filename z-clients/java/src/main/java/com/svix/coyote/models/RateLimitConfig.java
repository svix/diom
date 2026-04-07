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
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.svix.coyote.DurationMsSerializer;
import com.svix.coyote.DurationMsDeserializer;
import com.svix.coyote.ByteArrayAsIntArraySerializer;
import com.svix.coyote.ByteArrayAsIntArrayDeserializer;
import com.svix.coyote.Utils;
import java.time.Duration;
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
public class RateLimitConfig {
    @JsonProperty private Long capacity;
    @JsonProperty("refill_amount") private Long refillAmount;
    @JsonProperty("refill_interval_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration refillInterval;
    public RateLimitConfig() {}

    public RateLimitConfig capacity(Long capacity) {
        this.capacity = capacity;
        return this;
    }

    /**
    * Maximum capacity of the bucket
    *
     * @return capacity
     */
    @javax.annotation.Nonnull
    public Long getCapacity() {
        return capacity;
    }

    public void setCapacity(Long capacity) {
        this.capacity = capacity;
    }

    public RateLimitConfig refillAmount(Long refillAmount) {
        this.refillAmount = refillAmount;
        return this;
    }

    /**
    * Number of tokens to add per refill interval
    *
     * @return refillAmount
     */
    @javax.annotation.Nonnull
    public Long getRefillAmount() {
        return refillAmount;
    }

    public void setRefillAmount(Long refillAmount) {
        this.refillAmount = refillAmount;
    }

    public RateLimitConfig refillInterval(Duration refillInterval) {
        this.refillInterval = refillInterval;
        return this;
    }

    /**
    * Interval in milliseconds between refills (minimum 1 millisecond)
    *
     * @return refillInterval
     */
    @javax.annotation.Nullable
    public Duration getRefillInterval() {
        return refillInterval;
    }

    public void setRefillInterval(Duration refillInterval) {
        this.refillInterval = refillInterval;
    }

    /**
     * Create an instance of RateLimitConfig given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimitConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimitConfig
     */
    public static RateLimitConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimitConfig.class);
    }

    /**
     * Convert an instance of RateLimitConfig to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}