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
public class CreateStreamIn {
@JsonProperty("max_byte_size") private Long maxByteSize;
@JsonProperty private String name;
@JsonProperty("retention_period_seconds") private Long retentionPeriodSeconds;
public CreateStreamIn () {}

 public CreateStreamIn maxByteSize(Long maxByteSize) {
        this.maxByteSize = maxByteSize;
        return this;
    }

    /**
    * How many bytes in total the stream will retain before dropping data.
    *
     * @return maxByteSize
     */
    @javax.annotation.Nullable
     public Long getMaxByteSize() {
        return maxByteSize;
    }

     public void setMaxByteSize(Long maxByteSize) {
        this.maxByteSize = maxByteSize;
    }

     public CreateStreamIn name(String name) {
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

     public CreateStreamIn retentionPeriodSeconds(Long retentionPeriodSeconds) {
        this.retentionPeriodSeconds = retentionPeriodSeconds;
        return this;
    }

    /**
    * How long messages are retained in the stream before being permanently nuked.
    *
     * @return retentionPeriodSeconds
     */
    @javax.annotation.Nullable
     public Long getRetentionPeriodSeconds() {
        return retentionPeriodSeconds;
    }

     public void setRetentionPeriodSeconds(Long retentionPeriodSeconds) {
        this.retentionPeriodSeconds = retentionPeriodSeconds;
    }

    /**
     * Create an instance of CreateStreamIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CreateStreamIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to CreateStreamIn
     */
    public static CreateStreamIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CreateStreamIn.class);
    }

    /**
     * Convert an instance of CreateStreamIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}