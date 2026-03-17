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
public class Retention {
    @JsonProperty private Long millis;
    @JsonProperty private Long bytes;
    public Retention () {}

    public Retention millis(Long millis) {
        this.millis = millis;
        return this;
    }

    /**
    * Get millis
    *
     * @return millis
     */
    @javax.annotation.Nullable
    public Long getMillis() {
        return millis;
    }

    public void setMillis(Long millis) {
        this.millis = millis;
    }

    public Retention bytes(Long bytes) {
        this.bytes = bytes;
        return this;
    }

    /**
    * Get bytes
    *
     * @return bytes
     */
    @javax.annotation.Nullable
    public Long getBytes() {
        return bytes;
    }

    public void setBytes(Long bytes) {
        this.bytes = bytes;
    }
    /**
     * Create an instance of Retention given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of Retention
     * @throws JsonProcessingException if the JSON string is invalid with respect to Retention
     */
    public static Retention fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, Retention.class);
    }

    /**
     * Convert an instance of Retention to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}