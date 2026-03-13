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
public class RateLimitCheckIn {
@JsonProperty private String key;
@JsonProperty private Long tokens;
@JsonProperty private Long capacity;
@JsonProperty("refill_amount") private Long refillAmount;
@JsonProperty("refill_interval_millis") private Long refillIntervalMillis;
public RateLimitCheckIn () {}

 public RateLimitCheckIn key(String key) {
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

     public RateLimitCheckIn tokens(Long tokens) {
        this.tokens = tokens;
        return this;
    }

    /**
    * Number of tokens to consume (default: 1)
    *
     * @return tokens
     */
    @javax.annotation.Nullable
     public Long getTokens() {
        return tokens;
    }

     public void setTokens(Long tokens) {
        this.tokens = tokens;
    }

     public RateLimitCheckIn capacity(Long capacity) {
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

     public RateLimitCheckIn refillAmount(Long refillAmount) {
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

     public RateLimitCheckIn refillIntervalMillis(Long refillIntervalMillis) {
        this.refillIntervalMillis = refillIntervalMillis;
        return this;
    }

    /**
    * Interval in milliseconds between refills (minimum 1 millisecond)
    *
     * @return refillIntervalMillis
     */
    @javax.annotation.Nullable
     public Long getRefillIntervalMillis() {
        return refillIntervalMillis;
    }

     public void setRefillIntervalMillis(Long refillIntervalMillis) {
        this.refillIntervalMillis = refillIntervalMillis;
    }

    /**
     * Create an instance of RateLimitCheckIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimitCheckIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimitCheckIn
     */
    public static RateLimitCheckIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimitCheckIn.class);
    }

    /**
     * Convert an instance of RateLimitCheckIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}