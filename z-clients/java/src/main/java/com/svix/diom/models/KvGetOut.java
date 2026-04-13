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
public class KvGetOut {
    @JsonProperty @JsonSerialize(using = UnixTimestampMsSerializer.class) @JsonDeserialize(using = UnixTimestampMsDeserializer.class) private Instant expiry;
    @JsonProperty private byte[] value;
    @JsonProperty private Long version;
    public KvGetOut() {}

    public KvGetOut expiry(Instant expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
    * Time of expiry
    *
     * @return expiry
     */
    @javax.annotation.Nullable
    public Instant getExpiry() {
        return expiry;
    }

    public void setExpiry(Instant expiry) {
        this.expiry = expiry;
    }

    public KvGetOut value(byte[] value) {
        this.value = value;
        return this;
    }

    /**
    * Get value
    *
     * @return value
     */
    @javax.annotation.Nullable
    public byte[] getValue() {
        return value;
    }

    public void setValue(byte[] value) {
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
     * Create an instance of KvGetOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of KvGetOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to KvGetOut
     */
    public static KvGetOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, KvGetOut.class);
    }

    /**
     * Convert an instance of KvGetOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}