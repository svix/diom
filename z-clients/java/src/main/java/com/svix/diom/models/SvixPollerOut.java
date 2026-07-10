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
public class SvixPollerOut {
    @JsonProperty private String topic;
    @JsonProperty("poller_id") private String pollerId;
    @JsonProperty private String token;
    public SvixPollerOut() {}

    public SvixPollerOut topic(String topic) {
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

    public SvixPollerOut pollerId(String pollerId) {
        this.pollerId = pollerId;
        return this;
    }

    /**
    * Get pollerId
    *
     * @return pollerId
     */
    @javax.annotation.Nonnull
    public String getPollerId() {
        return pollerId;
    }

    public void setPollerId(String pollerId) {
        this.pollerId = pollerId;
    }

    public SvixPollerOut token(String token) {
        this.token = token;
        return this;
    }

    /**
    * The autoconfig token, obfuscated (e.g. `auto_v1_eyJh...fQ==`).
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

    /**
     * Create an instance of SvixPollerOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SvixPollerOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to SvixPollerOut
     */
    public static SvixPollerOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SvixPollerOut.class);
    }

    /**
     * Convert an instance of SvixPollerOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}