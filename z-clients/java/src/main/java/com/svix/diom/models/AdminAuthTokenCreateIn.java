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
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.svix.diom.DurationMsSerializer;
import com.svix.diom.DurationMsDeserializer;
import com.svix.diom.UnixTimestampMsSerializer;
import com.svix.diom.UnixTimestampMsDeserializer;
import com.svix.diom.Utils;
import java.time.Duration;
import java.time.Instant;
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
public class AdminAuthTokenCreateIn {
    @JsonProperty private String name;
    @JsonProperty private String role;
    @JsonProperty("expiry_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration expiry;
    @JsonProperty private Boolean enabled;
    public AdminAuthTokenCreateIn() {}

    public AdminAuthTokenCreateIn name(String name) {
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

    public AdminAuthTokenCreateIn role(String role) {
        this.role = role;
        return this;
    }

    /**
    * Get role
    *
     * @return role
     */
    @javax.annotation.Nonnull
    public String getRole() {
        return role;
    }

    public void setRole(String role) {
        this.role = role;
    }

    public AdminAuthTokenCreateIn expiry(Duration expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
    * Milliseconds from now until the token expires.
    *
     * @return expiry
     */
    @javax.annotation.Nullable
    public Duration getExpiry() {
        return expiry;
    }

    public void setExpiry(Duration expiry) {
        this.expiry = expiry;
    }

    public AdminAuthTokenCreateIn enabled(Boolean enabled) {
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
     * Create an instance of AdminAuthTokenCreateIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AdminAuthTokenCreateIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AdminAuthTokenCreateIn
     */
    public static AdminAuthTokenCreateIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AdminAuthTokenCreateIn.class);
    }

    /**
     * Convert an instance of AdminAuthTokenCreateIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}