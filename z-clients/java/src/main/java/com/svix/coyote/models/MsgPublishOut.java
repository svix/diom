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
public class MsgPublishOut {
    @JsonProperty private List<MsgPublishOutTopic> topics;
    public MsgPublishOut() {}

    public MsgPublishOut topics(List<MsgPublishOutTopic> topics) {
        this.topics = topics;
        return this;
    }

    public MsgPublishOut addTopicsItem(MsgPublishOutTopic topicsItem) {
        if (this.topics == null) {
            this.topics = new ArrayList<>();
        }
        this.topics.add(topicsItem);
        return this;
    }
    /**
    * Get topics
    *
     * @return topics
     */
    @javax.annotation.Nonnull
    public List<MsgPublishOutTopic> getTopics() {
        return topics;
    }

    public void setTopics(List<MsgPublishOutTopic> topics) {
        this.topics = topics;
    }

    /**
     * Create an instance of MsgPublishOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgPublishOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgPublishOut
     */
    public static MsgPublishOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgPublishOut.class);
    }

    /**
     * Convert an instance of MsgPublishOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}