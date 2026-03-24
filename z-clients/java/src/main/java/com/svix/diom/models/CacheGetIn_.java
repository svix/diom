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
import java.util.Objects;

@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class CacheGetIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String key;
    @JsonProperty private Consistency consistency;

    public CacheGetIn_(
        String namespace,
        String key,
        Consistency consistency
    ) {
        this.namespace = namespace;
        this.key = key;
        this.consistency = consistency;
    }

    /**
     * Create an instance of CacheGetIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CacheGetIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to CacheGetIn_
     */
    public static CacheGetIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CacheGetIn_.class);
    }

    /**
     * Convert an instance of CacheGetIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
