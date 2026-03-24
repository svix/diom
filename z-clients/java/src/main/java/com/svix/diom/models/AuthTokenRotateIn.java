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
public class AuthTokenRotateIn {
    @JsonProperty private String namespace;
    @JsonProperty private String id;
    @JsonProperty private String prefix;
    @JsonProperty private String suffix;
    @JsonProperty("expiry_ms") private Long expiryMs;
    public AuthTokenRotateIn() {}

    public AuthTokenRotateIn namespace(String namespace) {
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

    public AuthTokenRotateIn id(String id) {
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

    public AuthTokenRotateIn prefix(String prefix) {
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

    public AuthTokenRotateIn suffix(String suffix) {
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

    public AuthTokenRotateIn expiryMs(Long expiryMs) {
        this.expiryMs = expiryMs;
        return this;
    }

    /**
    * Milliseconds from now until the old token expires. `None` means expire immediately.
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

    /**
     * Create an instance of AuthTokenRotateIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AuthTokenRotateIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AuthTokenRotateIn
     */
    public static AuthTokenRotateIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AuthTokenRotateIn.class);
    }

    /**
     * Convert an instance of AuthTokenRotateIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}