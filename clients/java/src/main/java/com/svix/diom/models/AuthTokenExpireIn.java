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
public class AuthTokenExpireIn {
    @JsonProperty private String namespace;
    @JsonProperty private String id;
    @JsonProperty("expiry_millis") private Long expiryMillis;
    public AuthTokenExpireIn() {}

    public AuthTokenExpireIn namespace(String namespace) {
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

    public AuthTokenExpireIn id(String id) {
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

    public AuthTokenExpireIn expiryMillis(Long expiryMillis) {
        this.expiryMillis = expiryMillis;
        return this;
    }

    /**
    * Milliseconds from now until the token expires. `None` means expire immediately.
    *
     * @return expiryMillis
     */
    @javax.annotation.Nullable
    public Long getExpiryMillis() {
        return expiryMillis;
    }

    public void setExpiryMillis(Long expiryMillis) {
        this.expiryMillis = expiryMillis;
    }

    /**
     * Create an instance of AuthTokenExpireIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AuthTokenExpireIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AuthTokenExpireIn
     */
    public static AuthTokenExpireIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AuthTokenExpireIn.class);
    }

    /**
     * Convert an instance of AuthTokenExpireIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}