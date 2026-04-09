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
import com.svix.coyote.Utils;
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
public class QueueMsgOut {
    @JsonProperty("msg_id") private String msgId;
    @JsonProperty private byte[] value;
    @JsonProperty private Map<String,String> headers;
    @JsonProperty private OffsetDateTime timestamp;
    @JsonProperty("scheduled_at") private OffsetDateTime scheduledAt;
    public QueueMsgOut() {}

    public QueueMsgOut msgId(String msgId) {
        this.msgId = msgId;
        return this;
    }

    /**
    * Get msgId
    *
     * @return msgId
     */
    @javax.annotation.Nonnull
    public String getMsgId() {
        return msgId;
    }

    public void setMsgId(String msgId) {
        this.msgId = msgId;
    }

    public QueueMsgOut value(byte[] value) {
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

    public QueueMsgOut headers(Map<String,String> headers) {
        this.headers = headers;
        return this;
    }

    public QueueMsgOut putHeadersItem(String key, String headersItem) {
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

    public QueueMsgOut timestamp(OffsetDateTime timestamp) {
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

    public QueueMsgOut scheduledAt(OffsetDateTime scheduledAt) {
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
     * Create an instance of QueueMsgOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of QueueMsgOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to QueueMsgOut
     */
    public static QueueMsgOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, QueueMsgOut.class);
    }

    /**
     * Convert an instance of QueueMsgOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}