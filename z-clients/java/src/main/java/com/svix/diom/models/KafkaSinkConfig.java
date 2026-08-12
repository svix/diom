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
import com.svix.diom.UnixTimestampMsSerializer;
import com.svix.diom.UnixTimestampMsDeserializer;
import com.svix.diom.Utils;
import java.time.Duration;
import java.time.Instant;
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
public class KafkaSinkConfig {
    @JsonProperty("bootstrap_servers") private String bootstrapServers;
    @JsonProperty private String topic;
    @JsonProperty private String key;
    @JsonProperty private String value;
    @JsonProperty private Map<String, String> headers;
    @JsonProperty private KafkaSecurity security;
    public KafkaSinkConfig() {}

    public KafkaSinkConfig bootstrapServers(String bootstrapServers) {
        this.bootstrapServers = bootstrapServers;
        return this;
    }

    /**
    * Comma-separated `host:port` list of the target cluster's bootstrap brokers.
    *
     * @return bootstrapServers
     */
    @javax.annotation.Nonnull
    public String getBootstrapServers() {
        return bootstrapServers;
    }

    public void setBootstrapServers(String bootstrapServers) {
        this.bootstrapServers = bootstrapServers;
    }

    public KafkaSinkConfig topic(String topic) {
        this.topic = topic;
        return this;
    }

    /**
    * Destination Kafka topic.
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

    public KafkaSinkConfig key(String key) {
        this.key = key;
        return this;
    }

    /**
    * Templated record key rendered per-message. When absent, records are produced without a key.
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

    public KafkaSinkConfig value(String value) {
        this.value = value;
        return this;
    }

    /**
    * Templated record value. When absent, the raw message value bytes are produced unchanged.
    *
     * @return value
     */
    @javax.annotation.Nullable
    public String getValue() {
        return value;
    }

    public void setValue(String value) {
        this.value = value;
    }

    public KafkaSinkConfig headers(Map<String, String> headers) {
        this.headers = headers;
        return this;
    }

    public KafkaSinkConfig putHeadersItem(String key, String headersItem) {
        if (this.headers == null) {
            this.headers = new HashMap<>();
        }
        this.headers.put(key, headersItem);
        return this;
    }
    /**
    * Templated record headers merged on top of the message's own headers (which pass through by
default). A templated header overrides a passed-through one with the same name.
    *
     * @return headers
     */
    @javax.annotation.Nullable
    public Map<String, String> getHeaders() {
        return headers;
    }

    public void setHeaders(Map<String, String> headers) {
        this.headers = headers;
    }

    public KafkaSinkConfig security(KafkaSecurity security) {
        this.security = security;
        return this;
    }

    /**
    * Connection security (SASL and/or TLS). Defaults to none (PLAINTEXT).
    *
     * @return security
     */
    @javax.annotation.Nullable
    public KafkaSecurity getSecurity() {
        return security;
    }

    public void setSecurity(KafkaSecurity security) {
        this.security = security;
    }

    /**
     * Create an instance of KafkaSinkConfig given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of KafkaSinkConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to KafkaSinkConfig
     */
    public static KafkaSinkConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, KafkaSinkConfig.class);
    }

    /**
     * Convert an instance of KafkaSinkConfig to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}