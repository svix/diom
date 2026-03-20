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
import java.util.Objects;

@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class CacheDeleteIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String key;

    public CacheDeleteIn_(
        String namespace,
        String key
    ) {
        this.namespace = namespace;
        this.key = key;
    }

    /**
     * Create an instance of CacheDeleteIn_ given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CacheDeleteIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to CacheDeleteIn_
     */
    public static CacheDeleteIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CacheDeleteIn_.class);
    }

    /**
     * Convert an instance of CacheDeleteIn_ to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
