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
public class MsgStreamCommitIn_ {
    @JsonProperty private String namespace;
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty private Long offset;

    public MsgStreamCommitIn_(
        String namespace,
        String topic,
        String consumerGroup,
        Long offset
    ) {
        this.namespace = namespace;
        this.topic = topic;
        this.consumerGroup = consumerGroup;
        this.offset = offset;
    }

    /**
     * Create an instance of MsgStreamCommitIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgStreamCommitIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgStreamCommitIn_
     */
    public static MsgStreamCommitIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgStreamCommitIn_.class);
    }

    /**
     * Convert an instance of MsgStreamCommitIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
