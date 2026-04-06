// this file is @generated
package com.svix.diom.models;

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
import com.svix.diom.DurationMsSerializer;
import com.svix.diom.DurationMsDeserializer;
import com.svix.diom.Utils;
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
public class RateLimitGetRemainingOut {
    @JsonProperty private Long remaining;
    @JsonProperty("retry_after_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration retryAfter;
    public RateLimitGetRemainingOut() {}

    public RateLimitGetRemainingOut remaining(Long remaining) {
        this.remaining = remaining;
        return this;
    }

    /**
    * Number of tokens remaining
    *
     * @return remaining
     */
    @javax.annotation.Nonnull
    public Long getRemaining() {
        return remaining;
    }

    public void setRemaining(Long remaining) {
        this.remaining = remaining;
    }

    public RateLimitGetRemainingOut retryAfter(Duration retryAfter) {
        this.retryAfter = retryAfter;
        return this;
    }

    /**
    * Milliseconds until at least one token is available (only present when remaining is 0)
    *
     * @return retryAfter
     */
    @javax.annotation.Nullable
    public Duration getRetryAfter() {
        return retryAfter;
    }

    public void setRetryAfter(Duration retryAfter) {
        this.retryAfter = retryAfter;
    }

    /**
     * Create an instance of RateLimitGetRemainingOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimitGetRemainingOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimitGetRemainingOut
     */
    public static RateLimitGetRemainingOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimitGetRemainingOut.class);
    }

    /**
     * Convert an instance of RateLimitGetRemainingOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}