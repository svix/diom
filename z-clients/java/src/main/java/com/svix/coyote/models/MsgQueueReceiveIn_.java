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
import com.svix.coyote.Utils;
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
public class MsgQueueReceiveIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty("batch_size") private Short batchSize;
    @JsonProperty("lease_duration_ms") private Long leaseDurationMs;
    @JsonProperty("batch_wait_ms") private Long batchWaitMs;

    public MsgQueueReceiveIn_(
        String namespace,
        String topic,
        String consumerGroup,
        Short batchSize,
        Long leaseDurationMs,
        Long batchWaitMs
    ) {
        this.namespace = namespace;
        this.topic = topic;
        this.consumerGroup = consumerGroup;
        this.batchSize = batchSize;
        this.leaseDurationMs = leaseDurationMs;
        this.batchWaitMs = batchWaitMs;
    }

    /**
     * Create an instance of MsgQueueReceiveIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgQueueReceiveIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgQueueReceiveIn_
     */
    public static MsgQueueReceiveIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgQueueReceiveIn_.class);
    }

    /**
     * Convert an instance of MsgQueueReceiveIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
