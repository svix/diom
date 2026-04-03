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
import java.util.Objects;

@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MsgStreamReceiveIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty("batch_size") private Short batchSize;
    @JsonProperty("lease_duration_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration leaseDuration;
    @JsonProperty("default_starting_position") private SeekPosition defaultStartingPosition;
    @JsonProperty("batch_wait_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration batchWait;

    public MsgStreamReceiveIn_(
        String namespace,
        String topic,
        String consumerGroup,
        Short batchSize,
        Duration leaseDuration,
        SeekPosition defaultStartingPosition,
        Duration batchWait
    ) {
        this.namespace = namespace;
        this.topic = topic;
        this.consumerGroup = consumerGroup;
        this.batchSize = batchSize;
        this.leaseDuration = leaseDuration;
        this.defaultStartingPosition = defaultStartingPosition;
        this.batchWait = batchWait;
    }

    /**
     * Create an instance of MsgStreamReceiveIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgStreamReceiveIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgStreamReceiveIn_
     */
    public static MsgStreamReceiveIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgStreamReceiveIn_.class);
    }

    /**
     * Convert an instance of MsgStreamReceiveIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
