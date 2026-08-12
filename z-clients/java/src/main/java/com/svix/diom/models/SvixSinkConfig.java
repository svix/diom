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
public class SvixSinkConfig {
    @JsonProperty private String token;
    @JsonProperty("app_id") private String appId;
    @JsonProperty("event_type") private String eventType;
    @JsonProperty private String payload;
    @JsonProperty("idempotency_key") private String idempotencyKey;
    @JsonProperty("server_url") private String serverUrl;
    public SvixSinkConfig() {}

    public SvixSinkConfig token(String token) {
        this.token = token;
        return this;
    }

    /**
    * Svix API token, sent as the bearer credential. Obfuscated in list responses.
    *
     * @return token
     */
    @javax.annotation.Nonnull
    public String getToken() {
        return token;
    }

    public void setToken(String token) {
        this.token = token;
    }

    public SvixSinkConfig appId(String appId) {
        this.appId = appId;
        return this;
    }

    /**
    * Target Svix application. Can be optionally templated.
    *
     * @return appId
     */
    @javax.annotation.Nonnull
    public String getAppId() {
        return appId;
    }

    public void setAppId(String appId) {
        this.appId = appId;
    }

    public SvixSinkConfig eventType(String eventType) {
        this.eventType = eventType;
        return this;
    }

    /**
    * Svix event type. Can be optionally templated.
    *
     * @return eventType
     */
    @javax.annotation.Nonnull
    public String getEventType() {
        return eventType;
    }

    public void setEventType(String eventType) {
        this.eventType = eventType;
    }

    public SvixSinkConfig payload(String payload) {
        this.payload = payload;
        return this;
    }

    /**
    * Templated message payload. When absent, the raw message value bytes are used (must be JSON).
    *
     * @return payload
     */
    @javax.annotation.Nullable
    public String getPayload() {
        return payload;
    }

    public void setPayload(String payload) {
        this.payload = payload;
    }

    public SvixSinkConfig idempotencyKey(String idempotencyKey) {
        this.idempotencyKey = idempotencyKey;
        return this;
    }

    /**
    * Templated Svix `Idempotency-Key`. When absent or it renders to an empty string, a stable
key derived from the sink and message identity (namespace, topic, consumer_group, partition,
offset) is used so retries are de-duplicated by Svix.
    *
     * @return idempotencyKey
     */
    @javax.annotation.Nullable
    public String getIdempotencyKey() {
        return idempotencyKey;
    }

    public void setIdempotencyKey(String idempotencyKey) {
        this.idempotencyKey = idempotencyKey;
    }

    public SvixSinkConfig serverUrl(String serverUrl) {
        this.serverUrl = serverUrl;
        return this;
    }

    /**
    * Optional base URL override. When absent, the region is inferred from the token.
    *
     * @return serverUrl
     */
    @javax.annotation.Nullable
    public String getServerUrl() {
        return serverUrl;
    }

    public void setServerUrl(String serverUrl) {
        this.serverUrl = serverUrl;
    }

    /**
     * Create an instance of SvixSinkConfig given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SvixSinkConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to SvixSinkConfig
     */
    public static SvixSinkConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SvixSinkConfig.class);
    }

    /**
     * Convert an instance of SvixSinkConfig to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}