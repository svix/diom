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
public class Retention {
    @JsonProperty("period_ms") private Long periodMs;
    @JsonProperty("size_bytes") private Long sizeBytes;
    public Retention() {}

    public Retention periodMs(Long periodMs) {
        this.periodMs = periodMs;
        return this;
    }

    /**
    * Get periodMs
    *
     * @return periodMs
     */
    @javax.annotation.Nullable
    public Long getPeriodMs() {
        return periodMs;
    }

    public void setPeriodMs(Long periodMs) {
        this.periodMs = periodMs;
    }

    public Retention sizeBytes(Long sizeBytes) {
        this.sizeBytes = sizeBytes;
        return this;
    }

    /**
    * Get sizeBytes
    *
     * @return sizeBytes
     */
    @javax.annotation.Nullable
    public Long getSizeBytes() {
        return sizeBytes;
    }

    public void setSizeBytes(Long sizeBytes) {
        this.sizeBytes = sizeBytes;
    }

    /**
     * Create an instance of Retention given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of Retention
     * @throws JsonProcessingException if the JSON string is invalid with respect to Retention
     */
    public static Retention fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, Retention.class);
    }

    /**
     * Convert an instance of Retention to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}