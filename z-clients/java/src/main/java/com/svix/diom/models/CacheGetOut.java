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
public class CacheGetOut {
    @JsonProperty private OffsetDateTime expiry;
    @JsonProperty private byte[] value;
    public CacheGetOut() {}

    public CacheGetOut expiry(OffsetDateTime expiry) {
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

    public CacheGetOut value(byte[] value) {
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

    /**
     * Create an instance of CacheGetOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CacheGetOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to CacheGetOut
     */
    public static CacheGetOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CacheGetOut.class);
    }

    /**
     * Convert an instance of CacheGetOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}