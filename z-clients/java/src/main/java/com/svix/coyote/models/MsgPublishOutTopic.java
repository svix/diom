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
import com.svix.coyote.ByteArrayAsIntArraySerializer;
import com.svix.coyote.ByteArrayAsIntArrayDeserializer;
import com.svix.coyote.Utils;
import java.time.Duration;
import java.util.Map;
import java.util.Set;
import java.util.List;
import java.util.Optional;
import java.util.HashMap;
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
public class MsgPublishOutTopic {
    @JsonProperty private String topic;
    @JsonProperty("start_offset") private Long startOffset;
    @JsonProperty private Long offset;
    public MsgPublishOutTopic() {}

    public MsgPublishOutTopic topic(String topic) {
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

    public MsgPublishOutTopic startOffset(Long startOffset) {
        this.startOffset = startOffset;
        return this;
    }

    /**
    * Get startOffset
    *
     * @return startOffset
     */
    @javax.annotation.Nonnull
    public Long getStartOffset() {
        return startOffset;
    }

    public void setStartOffset(Long startOffset) {
        this.startOffset = startOffset;
    }

    public MsgPublishOutTopic offset(Long offset) {
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

    /**
     * Create an instance of MsgPublishOutTopic given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgPublishOutTopic
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgPublishOutTopic
     */
    public static MsgPublishOutTopic fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgPublishOutTopic.class);
    }

    /**
     * Convert an instance of MsgPublishOutTopic to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}