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
public class SinkOut {
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty("default_starting_position") private SeekPosition defaultStartingPosition;
    @JsonProperty("max_in_flight") private Integer maxInFlight;
    @JsonProperty private SinkConfig config;
    public SinkOut() {}

    public SinkOut topic(String topic) {
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

    public SinkOut consumerGroup(String consumerGroup) {
        this.consumerGroup = consumerGroup;
        return this;
    }

    /**
    * Get consumerGroup
    *
     * @return consumerGroup
     */
    @javax.annotation.Nonnull
    public String getConsumerGroup() {
        return consumerGroup;
    }

    public void setConsumerGroup(String consumerGroup) {
        this.consumerGroup = consumerGroup;
    }

    public SinkOut defaultStartingPosition(SeekPosition defaultStartingPosition) {
        this.defaultStartingPosition = defaultStartingPosition;
        return this;
    }

    /**
    * Where a freshly-created sink starts consuming the topic. Defaults to `earliest`.
    *
     * @return defaultStartingPosition
     */
    @javax.annotation.Nullable
    public SeekPosition getDefaultStartingPosition() {
        return defaultStartingPosition;
    }

    public void setDefaultStartingPosition(SeekPosition defaultStartingPosition) {
        this.defaultStartingPosition = defaultStartingPosition;
    }

    public SinkOut maxInFlight(Integer maxInFlight) {
        this.maxInFlight = maxInFlight;
        return this;
    }

    /**
    * At most how many concurrent requests will be sent to the Sink.
    *
     * @return maxInFlight
     */
    @javax.annotation.Nullable
    public Integer getMaxInFlight() {
        return maxInFlight;
    }

    public void setMaxInFlight(Integer maxInFlight) {
        this.maxInFlight = maxInFlight;
    }

    public SinkOut config(SinkConfig config) {
        this.config = config;
        return this;
    }

    /**
    * Get config
    *
     * @return config
     */
    @javax.annotation.Nonnull
    public SinkConfig getConfig() {
        return config;
    }

    public void setConfig(SinkConfig config) {
        this.config = config;
    }

    /**
     * Create an instance of SinkOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SinkOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to SinkOut
     */
    public static SinkOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SinkOut.class);
    }

    /**
     * Convert an instance of SinkOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}