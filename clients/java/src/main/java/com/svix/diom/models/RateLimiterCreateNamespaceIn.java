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
public class RateLimiterCreateNamespaceIn {
@JsonProperty private String name;
@JsonProperty("storage_type") private StorageType storageType;
@JsonProperty("max_storage_bytes") private Long maxStorageBytes;
public RateLimiterCreateNamespaceIn () {}

 public RateLimiterCreateNamespaceIn name(String name) {
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

     public RateLimiterCreateNamespaceIn storageType(StorageType storageType) {
        this.storageType = storageType;
        return this;
    }

    /**
    * Get storageType
    *
     * @return storageType
     */
    @javax.annotation.Nullable
     public StorageType getStorageType() {
        return storageType;
    }

     public void setStorageType(StorageType storageType) {
        this.storageType = storageType;
    }

     public RateLimiterCreateNamespaceIn maxStorageBytes(Long maxStorageBytes) {
        this.maxStorageBytes = maxStorageBytes;
        return this;
    }

    /**
    * Get maxStorageBytes
    *
     * @return maxStorageBytes
     */
    @javax.annotation.Nullable
     public Long getMaxStorageBytes() {
        return maxStorageBytes;
    }

     public void setMaxStorageBytes(Long maxStorageBytes) {
        this.maxStorageBytes = maxStorageBytes;
    }

    /**
     * Create an instance of RateLimiterCreateNamespaceIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimiterCreateNamespaceIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimiterCreateNamespaceIn
     */
    public static RateLimiterCreateNamespaceIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimiterCreateNamespaceIn.class);
    }

    /**
     * Convert an instance of RateLimiterCreateNamespaceIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}