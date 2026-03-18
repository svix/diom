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
import com.svix.diom.Utils;
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
    @JsonProperty("retry_after_millis") private Long retryAfterMillis;
    public RateLimitGetRemainingOut () {}

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

    public RateLimitGetRemainingOut retryAfterMillis(Long retryAfterMillis) {
        this.retryAfterMillis = retryAfterMillis;
        return this;
    }

    /**
    * Milliseconds until at least one token is available (only present when remaining is 0)
    *
     * @return retryAfterMillis
     */
    @javax.annotation.Nullable
    public Long getRetryAfterMillis() {
        return retryAfterMillis;
    }

    public void setRetryAfterMillis(Long retryAfterMillis) {
        this.retryAfterMillis = retryAfterMillis;
    }

    /**
     * Create an instance of RateLimitGetRemainingOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimitGetRemainingOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimitGetRemainingOut
     */
    public static RateLimitGetRemainingOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimitGetRemainingOut.class);
    }

    /**
     * Convert an instance of RateLimitGetRemainingOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}