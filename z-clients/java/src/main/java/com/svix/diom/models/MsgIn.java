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
public class MsgIn {
    @JsonProperty @JsonSerialize(using = ByteArrayAsIntArraySerializer.class) @JsonDeserialize(using = ByteArrayAsIntArrayDeserializer.class) private byte[] value;
    @JsonProperty private Map<String,String> headers;
    @JsonProperty private String key;
    @JsonProperty("delay_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration delay;
    public MsgIn() {}

    public MsgIn value(byte[] value) {
        this.value = value;
        return this;
    }

    /**
    * Get value
    *
     * @return value
     */
    @javax.annotation.Nonnull
    public byte[] getValue() {
        return value;
    }

    public void setValue(byte[] value) {
        this.value = value;
    }

    public MsgIn headers(Map<String,String> headers) {
        this.headers = headers;
        return this;
    }

    public MsgIn putHeadersItem(String key, String headersItem) {
        if (this.headers == null) {
            this.headers = new HashMap<>();
        }
        this.headers.put(key, headersItem);
        return this;
    }
    /**
    * Get headers
    *
     * @return headers
     */
    @javax.annotation.Nullable
    public Map<String,String> getHeaders() {
        return headers;
    }

    public void setHeaders(Map<String,String> headers) {
        this.headers = headers;
    }

    public MsgIn key(String key) {
        this.key = key;
        return this;
    }

    /**
    * Optional partition key.

Messages with the same key are routed to the same partition.
    *
     * @return key
     */
    @javax.annotation.Nullable
    public String getKey() {
        return key;
    }

    public void setKey(String key) {
        this.key = key;
    }

    public MsgIn delay(Duration delay) {
        this.delay = delay;
        return this;
    }

    /**
    * Optional delay in milliseconds.

The message will not be delivered to queue consumers
until the delay has elapsed from the time of publish.
    *
     * @return delay
     */
    @javax.annotation.Nullable
    public Duration getDelay() {
        return delay;
    }

    public void setDelay(Duration delay) {
        this.delay = delay;
    }

    /**
     * Create an instance of MsgIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgIn
     */
    public static MsgIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgIn.class);
    }

    /**
     * Convert an instance of MsgIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}