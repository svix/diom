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
import java.util.Objects;

@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class IdempotencyCompleteIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String key;
    @JsonProperty @JsonSerialize(using = ByteArrayAsIntArraySerializer.class) @JsonDeserialize(using = ByteArrayAsIntArrayDeserializer.class) private byte[] response;
    @JsonProperty private Map<String,String> context;
    @JsonProperty("ttl_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration ttl;

    public IdempotencyCompleteIn_(
        String namespace,
        String key,
        byte[] response,
        Map<String,String> context,
        Duration ttl
    ) {
        this.namespace = namespace;
        this.key = key;
        this.response = response;
        this.context = context;
        this.ttl = ttl;
    }

    /**
     * Create an instance of IdempotencyCompleteIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IdempotencyCompleteIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to IdempotencyCompleteIn_
     */
    public static IdempotencyCompleteIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IdempotencyCompleteIn_.class);
    }

    /**
     * Convert an instance of IdempotencyCompleteIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
