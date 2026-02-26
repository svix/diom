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
public class CreateStreamOut {
@JsonProperty("created_at") private OffsetDateTime createdAt;
@JsonProperty("max_byte_size") private Long maxByteSize;
@JsonProperty private String name;
@JsonProperty("retention_period_seconds") private Long retentionPeriodSeconds;
@JsonProperty("updated_at") private OffsetDateTime updatedAt;
public CreateStreamOut () {}

 public CreateStreamOut createdAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
        return this;
    }

    /**
    * Get createdAt
    *
     * @return createdAt
     */
    @javax.annotation.Nonnull
     public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

     public void setCreatedAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
    }

     public CreateStreamOut maxByteSize(Long maxByteSize) {
        this.maxByteSize = maxByteSize;
        return this;
    }

    /**
    * Get maxByteSize
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

     public CreateStreamOut name(String name) {
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

     public CreateStreamOut retentionPeriodSeconds(Long retentionPeriodSeconds) {
        this.retentionPeriodSeconds = retentionPeriodSeconds;
        return this;
    }

    /**
    * Get retentionPeriodSeconds
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

     public CreateStreamOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
        return this;
    }

    /**
    * Get updatedAt
    *
     * @return updatedAt
     */
    @javax.annotation.Nonnull
     public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

     public void setUpdatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    /**
     * Create an instance of CreateStreamOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CreateStreamOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to CreateStreamOut
     */
    public static CreateStreamOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CreateStreamOut.class);
    }

    /**
     * Convert an instance of CreateStreamOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}