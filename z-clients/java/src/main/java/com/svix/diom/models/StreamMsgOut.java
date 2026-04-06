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
public class StreamMsgOut {
    @JsonProperty private Long offset;
    @JsonProperty private String topic;
    @JsonProperty private List<Byte> value;
    @JsonProperty private Map<String,String> headers;
    @JsonProperty private OffsetDateTime timestamp;
    @JsonProperty("scheduled_at") private OffsetDateTime scheduledAt;
    public StreamMsgOut() {}

    public StreamMsgOut offset(Long offset) {
        this.offset = offset;
        return this;
    }

    /**
    * Get offset
    *
     * @return offset
     */
    @javax.annotation.Nonnull
    public Long getOffset() {
        return offset;
    }

    public void setOffset(Long offset) {
        this.offset = offset;
    }

    public StreamMsgOut topic(String topic) {
        this.topic = topic;
        return this;
    }

    /**
    * Get topic
    *
     * @return topic
     */
    @javax.annotation.Nonnull
    public String getTopic() {
        return topic;
    }

    public void setTopic(String topic) {
        this.topic = topic;
    }

    public StreamMsgOut value(List<Byte> value) {
        this.value = value;
        return this;
    }

    public StreamMsgOut addValueItem(Byte valueItem) {
        if (this.value == null) {
            this.value = new ArrayList<>();
        }
        this.value.add(valueItem);
        return this;
    }
    /**
    * Get value
    *
     * @return value
     */
    @javax.annotation.Nonnull
    public List<Byte> getValue() {
        return value;
    }

    public void setValue(List<Byte> value) {
        this.value = value;
    }

    public StreamMsgOut headers(Map<String,String> headers) {
        this.headers = headers;
        return this;
    }

    public StreamMsgOut putHeadersItem(String key, String headersItem) {
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

    public StreamMsgOut timestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
        return this;
    }

    /**
    * Get timestamp
    *
     * @return timestamp
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
    }

    public StreamMsgOut scheduledAt(OffsetDateTime scheduledAt) {
        this.scheduledAt = scheduledAt;
        return this;
    }

    /**
    * Get scheduledAt
    *
     * @return scheduledAt
     */
    @javax.annotation.Nullable
    public OffsetDateTime getScheduledAt() {
        return scheduledAt;
    }

    public void setScheduledAt(OffsetDateTime scheduledAt) {
        this.scheduledAt = scheduledAt;
    }

    /**
     * Create an instance of StreamMsgOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of StreamMsgOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to StreamMsgOut
     */
    public static StreamMsgOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamMsgOut.class);
    }

    /**
     * Convert an instance of StreamMsgOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}