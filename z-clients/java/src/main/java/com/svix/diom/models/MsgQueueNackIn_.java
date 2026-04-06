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
import java.util.Objects;

@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MsgQueueNackIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty("msg_ids") private List<String> msgIds;

    public MsgQueueNackIn_(
        String namespace,
        String topic,
        String consumerGroup,
        List<String> msgIds
    ) {
        this.namespace = namespace;
        this.topic = topic;
        this.consumerGroup = consumerGroup;
        this.msgIds = msgIds;
    }

    /**
     * Create an instance of MsgQueueNackIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgQueueNackIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgQueueNackIn_
     */
    public static MsgQueueNackIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgQueueNackIn_.class);
    }

    /**
     * Convert an instance of MsgQueueNackIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
