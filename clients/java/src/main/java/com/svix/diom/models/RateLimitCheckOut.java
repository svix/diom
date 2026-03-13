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
@JsonAutoDetect(getterVisibility = Visibility.NONE,setterVisibility = Visibility.NONE)
public class RateLimitCheckOut {
@JsonProperty private RateLimitStatus status;
@JsonProperty private Long remaining;
@JsonProperty("retry_after") private Long retryAfter;
public RateLimitCheckOut () {}

 public RateLimitCheckOut status(RateLimitStatus status) {
        this.status = status;
        return this;
    }

    /**
    * Whether the request is allowed
    *
     * @return status
     */
    @javax.annotation.Nonnull
     public RateLimitStatus getStatus() {
        return status;
    }

     public void setStatus(RateLimitStatus status) {
        this.status = status;
    }

     public RateLimitCheckOut remaining(Long remaining) {
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

     public RateLimitCheckOut retryAfter(Long retryAfter) {
        this.retryAfter = retryAfter;
        return this;
    }

    /**
    * Seconds until enough tokens are available (only present when allowed is false)
    *
     * @return retryAfter
     */
    @javax.annotation.Nullable
     public Long getRetryAfter() {
        return retryAfter;
    }

     public void setRetryAfter(Long retryAfter) {
        this.retryAfter = retryAfter;
    }

    /**
     * Create an instance of RateLimitCheckOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimitCheckOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimitCheckOut
     */
    public static RateLimitCheckOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimitCheckOut.class);
    }

    /**
     * Convert an instance of RateLimitCheckOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}