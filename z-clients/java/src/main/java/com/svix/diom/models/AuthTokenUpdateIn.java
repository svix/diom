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
public class AuthTokenUpdateIn {
    @JsonProperty private String namespace;
    @JsonProperty private String id;
    @JsonProperty private String name;
    @JsonProperty("expiry_ms") private Long expiryMs;
    @JsonProperty private Map<String,String> metadata;
    @JsonProperty private List<String> scopes;
    @JsonProperty private Boolean enabled;
    public AuthTokenUpdateIn() {}

    public AuthTokenUpdateIn namespace(String namespace) {
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

    public AuthTokenUpdateIn id(String id) {
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

    public AuthTokenUpdateIn name(String name) {
        this.name = name;
        return this;
    }

    /**
    * Get name
    *
     * @return name
     */
    @javax.annotation.Nullable
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public AuthTokenUpdateIn expiryMs(Long expiryMs) {
        this.expiryMs = expiryMs;
        return this;
    }

    /**
    * Get expiryMs
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

    public AuthTokenUpdateIn metadata(Map<String,String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public AuthTokenUpdateIn putMetadataItem(String key, String metadataItem) {
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

    public AuthTokenUpdateIn scopes(List<String> scopes) {
        this.scopes = scopes;
        return this;
    }

    public AuthTokenUpdateIn addScopesItem(String scopesItem) {
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

    public AuthTokenUpdateIn enabled(Boolean enabled) {
        this.enabled = enabled;
        return this;
    }

    /**
    * Get enabled
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
     * Create an instance of AuthTokenUpdateIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AuthTokenUpdateIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AuthTokenUpdateIn
     */
    public static AuthTokenUpdateIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AuthTokenUpdateIn.class);
    }

    /**
     * Convert an instance of AuthTokenUpdateIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}