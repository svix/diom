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
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.svix.coyote.DurationMsSerializer;
import com.svix.coyote.DurationMsDeserializer;
import com.svix.coyote.Utils;
import java.time.Duration;
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
    @JsonProperty private List<Byte> response;
    @JsonProperty("ttl_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration ttl;
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

    public IdempotencyCompleteIn ttl(Duration ttl) {
        this.ttl = ttl;
        return this;
    }

    /**
    * TTL in milliseconds for the cached response
    *
     * @return ttl
     */
    @javax.annotation.Nonnull
    public Duration getTtl() {
        return ttl;
    }

    public void setTtl(Duration ttl) {
        this.ttl = ttl;
    }
}