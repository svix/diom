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
import com.svix.coyote.ByteArrayAsIntArraySerializer;
import com.svix.coyote.ByteArrayAsIntArrayDeserializer;
import com.svix.coyote.Utils;
import java.time.Duration;
import java.util.Map;
import java.util.Set;
import java.util.List;
import java.util.Optional;
import java.util.HashMap;
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
public class RateLimitCreateNamespaceOut {
    @JsonProperty private String name;
    @JsonProperty private Long created;
    @JsonProperty private Long updated;
    public RateLimitCreateNamespaceOut() {}

    public RateLimitCreateNamespaceOut name(String name) {
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

    public RateLimitCreateNamespaceOut created(Long created) {
        this.created = created;
        return this;
    }

    /**
    * Get created
    *
     * @return created
     */
    @javax.annotation.Nonnull
    public Long getCreated() {
        return created;
    }

    public void setCreated(Long created) {
        this.created = created;
    }

    public RateLimitCreateNamespaceOut updated(Long updated) {
        this.updated = updated;
        return this;
    }

    /**
    * Get updated
    *
     * @return updated
     */
    @javax.annotation.Nonnull
    public Long getUpdated() {
        return updated;
    }

    public void setUpdated(Long updated) {
        this.updated = updated;
    }

    /**
     * Create an instance of RateLimitCreateNamespaceOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RateLimitCreateNamespaceOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to RateLimitCreateNamespaceOut
     */
    public static RateLimitCreateNamespaceOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimitCreateNamespaceOut.class);
    }

    /**
     * Convert an instance of RateLimitCreateNamespaceOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}