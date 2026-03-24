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
public class KvSetIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String key;
    @JsonProperty private List<Byte> value;
    @JsonProperty private Long ttl;
    @JsonProperty private OperationBehavior behavior;
    @JsonProperty private Long version;

    public KvSetIn_(
        String namespace,
        String key,
        List<Byte> value,
        Long ttl,
        OperationBehavior behavior,
        Long version
    ) {
        this.namespace = namespace;
        this.key = key;
        this.value = value;
        this.ttl = ttl;
        this.behavior = behavior;
        this.version = version;
    }

    /**
     * Create an instance of KvSetIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of KvSetIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to KvSetIn_
     */
    public static KvSetIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, KvSetIn_.class);
    }

    /**
     * Convert an instance of KvSetIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
