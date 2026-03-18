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
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class AuthTokenOut {
    @JsonProperty private String id;
    @JsonProperty private String name;
    @JsonProperty("created_at") private OffsetDateTime createdAt;
    @JsonProperty("updated_at") private OffsetDateTime updatedAt;
    @JsonProperty private OffsetDateTime expiry;
    @JsonProperty private Map<String,String> metadata;
    @JsonProperty("owner_id") private String ownerId;
    @JsonProperty private List<String> scopes;
    @JsonProperty private Boolean enabled;
    public AuthTokenOut() {}

    public AuthTokenOut id(String id) {
        this.id = id;
        return this;
    }

    /**
    * Get id
    *
     * @return id
     */
    @javax.annotation.Nonnull
    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public AuthTokenOut name(String name) {
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

    public AuthTokenOut createdAt(OffsetDateTime createdAt) {
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

    public AuthTokenOut updatedAt(OffsetDateTime updatedAt) {
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

    public AuthTokenOut expiry(OffsetDateTime expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
    * Get expiry
    *
     * @return expiry
     */
    @javax.annotation.Nullable
    public OffsetDateTime getExpiry() {
        return expiry;
    }

    public void setExpiry(OffsetDateTime expiry) {
        this.expiry = expiry;
    }

    public AuthTokenOut metadata(Map<String,String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public AuthTokenOut putMetadataItem(String key, String metadataItem) {
        if (this.metadata == null) {
            this.metadata = new HashMap<>();
        }
        this.metadata.put(key, metadataItem);
        return this;
    }
    /**
    * Get metadata
    *
     * @return metadata
     */
    @javax.annotation.Nonnull
    public Map<String,String> getMetadata() {
        return metadata;
    }

    public void setMetadata(Map<String,String> metadata) {
        this.metadata = metadata;
    }

    public AuthTokenOut ownerId(String ownerId) {
        this.ownerId = ownerId;
        return this;
    }

    /**
    * Get ownerId
    *
     * @return ownerId
     */
    @javax.annotation.Nonnull
    public String getOwnerId() {
        return ownerId;
    }

    public void setOwnerId(String ownerId) {
        this.ownerId = ownerId;
    }

    public AuthTokenOut scopes(List<String> scopes) {
        this.scopes = scopes;
        return this;
    }

    public AuthTokenOut addScopesItem(String scopesItem) {
        if (this.scopes == null) {
            this.scopes = new ArrayList<>();
        }
        this.scopes.add(scopesItem);
        return this;
    }
    /**
    * Get scopes
    *
     * @return scopes
     */
    @javax.annotation.Nonnull
    public List<String> getScopes() {
        return scopes;
    }

    public void setScopes(List<String> scopes) {
        this.scopes = scopes;
    }

    public AuthTokenOut enabled(Boolean enabled) {
        this.enabled = enabled;
        return this;
    }

    /**
    * Whether this token is currently enabled.
    *
     * @return enabled
     */
    @javax.annotation.Nonnull
    public Boolean getEnabled() {
        return enabled;
    }

    public void setEnabled(Boolean enabled) {
        this.enabled = enabled;
    }

    /**
     * Create an instance of AuthTokenOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AuthTokenOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to AuthTokenOut
     */
    public static AuthTokenOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AuthTokenOut.class);
    }

    /**
     * Convert an instance of AuthTokenOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}