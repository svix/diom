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
import java.util.Objects;

@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class KvSetIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String key;
    @JsonProperty @JsonSerialize(using = ByteArrayAsIntArraySerializer.class) @JsonDeserialize(using = ByteArrayAsIntArrayDeserializer.class) private byte[] value;
    @JsonProperty("ttl_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration ttl;
    @JsonProperty private OperationBehavior behavior;
    @JsonProperty private Long version;

    public KvSetIn_(
        String namespace,
        String key,
        byte[] value,
        Duration ttl,
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
