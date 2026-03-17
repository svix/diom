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
public class KvGetOut {
    @JsonProperty private OffsetDateTime expiry;
    @JsonProperty private List<Byte> value;
    @JsonProperty private Long version;
    public KvGetOut () {}

    public KvGetOut expiry(OffsetDateTime expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
    * Time of expiry
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

    public KvGetOut value(List<Byte> value) {
        this.value = value;
        return this;
    }

    public KvGetOut addValueItem(Byte valueItem) {
        if (this.value == null) {
            this.value = new ArrayList<>();
        }
        this.value.add(valueItem);
        return this;
    }
    /**
    * Get value
    *
     * @return value
     */
    @javax.annotation.Nullable
    public List<Byte> getValue() {
        return value;
    }

    public void setValue(List<Byte> value) {
        this.value = value;
    }

    public KvGetOut version(Long version) {
        this.version = version;
        return this;
    }

    /**
    * Opaque version token for optimistic concurrency control.
Pass as `version` in a subsequent `set` to perform a conditional write.
    *
     * @return version
     */
    @javax.annotation.Nonnull
    public Long getVersion() {
        return version;
    }

    public void setVersion(Long version) {
        this.version = version;
    }
    /**
     * Create an instance of KvGetOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of KvGetOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to KvGetOut
     */
    public static KvGetOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, KvGetOut.class);
    }

    /**
     * Convert an instance of KvGetOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}