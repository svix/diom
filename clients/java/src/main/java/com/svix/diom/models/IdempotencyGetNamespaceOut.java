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
public class IdempotencyGetNamespaceOut {
@JsonProperty("created_at") private OffsetDateTime createdAt;
@JsonProperty("max_storage_bytes") private Long maxStorageBytes;
@JsonProperty private String name;
@JsonProperty("updated_at") private OffsetDateTime updatedAt;
public IdempotencyGetNamespaceOut () {}

 public IdempotencyGetNamespaceOut createdAt(OffsetDateTime createdAt) {
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

     public IdempotencyGetNamespaceOut maxStorageBytes(Long maxStorageBytes) {
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

     public IdempotencyGetNamespaceOut name(String name) {
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

     public IdempotencyGetNamespaceOut updatedAt(OffsetDateTime updatedAt) {
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
     * Create an instance of IdempotencyGetNamespaceOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IdempotencyGetNamespaceOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to IdempotencyGetNamespaceOut
     */
    public static IdempotencyGetNamespaceOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IdempotencyGetNamespaceOut.class);
    }

    /**
     * Convert an instance of IdempotencyGetNamespaceOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}