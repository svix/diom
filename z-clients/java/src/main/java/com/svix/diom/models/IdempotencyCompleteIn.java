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
public class IdempotencyCompleteIn {
    @JsonProperty private String namespace;
    @JsonProperty private String key;
    @JsonProperty private List<Byte> response;
    @JsonProperty("ttl_ms") private Long ttlMs;
    public IdempotencyCompleteIn() {}

    public IdempotencyCompleteIn namespace(String namespace) {
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

    public IdempotencyCompleteIn response(List<Byte> response) {
        this.response = response;
        return this;
    }

    public IdempotencyCompleteIn addResponseItem(Byte responseItem) {
        if (this.response == null) {
            this.response = new ArrayList<>();
        }
        this.response.add(responseItem);
        return this;
    }
    /**
    * The response to cache
    *
     * @return response
     */
    @javax.annotation.Nonnull
    public List<Byte> getResponse() {
        return response;
    }

    public void setResponse(List<Byte> response) {
        this.response = response;
    }

    public IdempotencyCompleteIn ttlMs(Long ttlMs) {
        this.ttlMs = ttlMs;
        return this;
    }

    /**
    * TTL in milliseconds for the cached response
    *
     * @return ttlMs
     */
    @javax.annotation.Nonnull
    public Long getTtlMs() {
        return ttlMs;
    }

    public void setTtlMs(Long ttlMs) {
        this.ttlMs = ttlMs;
    }
}