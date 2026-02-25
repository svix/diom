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
@JsonAutoDetect(getterVisibility = Visibility.NONE,setterVisibility = Visibility.NONE)
public class FetchFromStreamIn {
@JsonProperty("batch_size") private Short batchSize;
@JsonProperty("consumer_group") private String consumerGroup;
@JsonProperty private String name;
@JsonProperty("visibility_timeout_seconds") private Long visibilityTimeoutSeconds;
public FetchFromStreamIn () {}

 public FetchFromStreamIn batchSize(Short batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    /**
    * How many messages to read from the stream.
    *
     * @return batchSize
     */
    @javax.annotation.Nonnull
     public Short getBatchSize() {
        return batchSize;
    }

     public void setBatchSize(Short batchSize) {
        this.batchSize = batchSize;
    }

     public FetchFromStreamIn consumerGroup(String consumerGroup) {
        this.consumerGroup = consumerGroup;
        return this;
    }

    /**
    * Get consumerGroup
    *
     * @return consumerGroup
     */
    @javax.annotation.Nonnull
     public String getConsumerGroup() {
        return consumerGroup;
    }

     public void setConsumerGroup(String consumerGroup) {
        this.consumerGroup = consumerGroup;
    }

     public FetchFromStreamIn name(String name) {
        this.name = name;
        return this;
    }

    /**
    * Get name
    *
     * @return name
     */
    @javax.annotation.Nonnull
     public String getName() {
        return name;
    }

     public void setName(String name) {
        this.name = name;
    }

     public FetchFromStreamIn visibilityTimeoutSeconds(Long visibilityTimeoutSeconds) {
        this.visibilityTimeoutSeconds = visibilityTimeoutSeconds;
        return this;
    }

    /**
    * How long messages are locked by the consumer.
    *
     * @return visibilityTimeoutSeconds
     */
    @javax.annotation.Nonnull
     public Long getVisibilityTimeoutSeconds() {
        return visibilityTimeoutSeconds;
    }

     public void setVisibilityTimeoutSeconds(Long visibilityTimeoutSeconds) {
        this.visibilityTimeoutSeconds = visibilityTimeoutSeconds;
    }

    /**
     * Create an instance of FetchFromStreamIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of FetchFromStreamIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to FetchFromStreamIn
     */
    public static FetchFromStreamIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, FetchFromStreamIn.class);
    }

    /**
     * Convert an instance of FetchFromStreamIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}