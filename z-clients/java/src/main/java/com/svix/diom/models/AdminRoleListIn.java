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
import com.svix.diom.ByteArrayAsIntArraySerializer;
import com.svix.diom.ByteArrayAsIntArrayDeserializer;
import com.svix.diom.Utils;
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
public class AdminRoleListIn {
    @JsonProperty private Long limit;
    @JsonProperty private String iterator;
    public AdminRoleListIn() {}

    public AdminRoleListIn limit(Long limit) {
        this.limit = limit;
        return this;
    }

    /**
    * Limit the number of returned items
    *
     * @return limit
     */
    @javax.annotation.Nullable
    public Long getLimit() {
        return limit;
    }

    public void setLimit(Long limit) {
        this.limit = limit;
    }

    public AdminRoleListIn iterator(String iterator) {
        this.iterator = iterator;
        return this;
    }

    /**
    * The iterator returned from a prior invocation
    *
     * @return iterator
     */
    @javax.annotation.Nullable
    public String getIterator() {
        return iterator;
    }

    public void setIterator(String iterator) {
        this.iterator = iterator;
    }

    /**
     * Create an instance of AdminRoleListIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AdminRoleListIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AdminRoleListIn
     */
    public static AdminRoleListIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AdminRoleListIn.class);
    }

    /**
     * Convert an instance of AdminRoleListIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}