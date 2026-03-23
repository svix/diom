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
public class AuthTokenCreateIn {
    @JsonProperty private String namespace;
    @JsonProperty private String name;
    @JsonProperty private String prefix;
    @JsonProperty private String suffix;
    @JsonProperty("expiry_ms") private Long expiryMs;
    @JsonProperty private Map<String,String> metadata;
    @JsonProperty("owner_id") private String ownerId;
    @JsonProperty private List<String> scopes;
    @JsonProperty private Boolean enabled;
    public AuthTokenCreateIn() {}

    public AuthTokenCreateIn namespace(String namespace) {
        this.namespace = namespace;
        return this;
    }

    /**
    * Get namespace
    *
     * @return namespace
     */
    @javax.annotation.Nullable
    public String getNamespace() {
        return namespace;
    }

    public void setNamespace(String namespace) {
        this.namespace = namespace;
    }

    public AuthTokenCreateIn name(String name) {
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

    public AuthTokenCreateIn prefix(String prefix) {
        this.prefix = prefix;
        return this;
    }

    /**
    * Get prefix
    *
     * @return prefix
     */
    @javax.annotation.Nullable
    public String getPrefix() {
        return prefix;
    }

    public void setPrefix(String prefix) {
        this.prefix = prefix;
    }

    public AuthTokenCreateIn suffix(String suffix) {
        this.suffix = suffix;
        return this;
    }

    /**
    * Get suffix
    *
     * @return suffix
     */
    @javax.annotation.Nullable
    public String getSuffix() {
        return suffix;
    }

    public void setSuffix(String suffix) {
        this.suffix = suffix;
    }

    public AuthTokenCreateIn expiryMs(Long expiryMs) {
        this.expiryMs = expiryMs;
        return this;
    }

    /**
    * Milliseconds from now until the token expires.
    *
     * @return expiryMs
     */
    @javax.annotation.Nullable
    public Long getExpiryMs() {
        return expiryMs;
    }

    public void setExpiryMs(Long expiryMs) {
        this.expiryMs = expiryMs;
    }

    public AuthTokenCreateIn metadata(Map<String,String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public AuthTokenCreateIn putMetadataItem(String key, String metadataItem) {
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
    @javax.annotation.Nullable
    public Map<String,String> getMetadata() {
        return metadata;
    }

    public void setMetadata(Map<String,String> metadata) {
        this.metadata = metadata;
    }

    public AuthTokenCreateIn ownerId(String ownerId) {
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

    public AuthTokenCreateIn scopes(List<String> scopes) {
        this.scopes = scopes;
        return this;
    }

    public AuthTokenCreateIn addScopesItem(String scopesItem) {
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
    @javax.annotation.Nullable
    public List<String> getScopes() {
        return scopes;
    }

    public void setScopes(List<String> scopes) {
        this.scopes = scopes;
    }

    public AuthTokenCreateIn enabled(Boolean enabled) {
        this.enabled = enabled;
        return this;
    }

    /**
    * Whether the token is enabled. Defaults to `true`.
    *
     * @return enabled
     */
    @javax.annotation.Nullable
    public Boolean getEnabled() {
        return enabled;
    }

    public void setEnabled(Boolean enabled) {
        this.enabled = enabled;
    }

    /**
     * Create an instance of AuthTokenCreateIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AuthTokenCreateIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AuthTokenCreateIn
     */
    public static AuthTokenCreateIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AuthTokenCreateIn.class);
    }

    /**
     * Convert an instance of AuthTokenCreateIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}