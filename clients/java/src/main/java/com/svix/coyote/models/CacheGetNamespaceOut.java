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
public class CacheGetNamespaceOut {
@JsonProperty("created_at") private OffsetDateTime createdAt;
@JsonProperty("eviction_policy") private EvictionPolicy evictionPolicy;
@JsonProperty("max_storage_bytes") private Long maxStorageBytes;
@JsonProperty private String name;
@JsonProperty("storage_type") private StorageType storageType;
@JsonProperty("updated_at") private OffsetDateTime updatedAt;
public CacheGetNamespaceOut () {}

 public CacheGetNamespaceOut createdAt(OffsetDateTime createdAt) {
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

     public CacheGetNamespaceOut evictionPolicy(EvictionPolicy evictionPolicy) {
        this.evictionPolicy = evictionPolicy;
        return this;
    }

    /**
    * Get evictionPolicy
    *
     * @return evictionPolicy
     */
    @javax.annotation.Nonnull
     public EvictionPolicy getEvictionPolicy() {
        return evictionPolicy;
    }

     public void setEvictionPolicy(EvictionPolicy evictionPolicy) {
        this.evictionPolicy = evictionPolicy;
    }

     public CacheGetNamespaceOut maxStorageBytes(Long maxStorageBytes) {
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

     public CacheGetNamespaceOut name(String name) {
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

     public CacheGetNamespaceOut storageType(StorageType storageType) {
        this.storageType = storageType;
        return this;
    }

    /**
    * Get storageType
    *
     * @return storageType
     */
    @javax.annotation.Nonnull
     public StorageType getStorageType() {
        return storageType;
    }

     public void setStorageType(StorageType storageType) {
        this.storageType = storageType;
    }

     public CacheGetNamespaceOut updatedAt(OffsetDateTime updatedAt) {
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
     * Create an instance of CacheGetNamespaceOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CacheGetNamespaceOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to CacheGetNamespaceOut
     */
    public static CacheGetNamespaceOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CacheGetNamespaceOut.class);
    }

    /**
     * Convert an instance of CacheGetNamespaceOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}