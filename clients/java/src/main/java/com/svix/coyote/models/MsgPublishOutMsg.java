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
import java.net.URI;
import java.util.Objects;
import lombok.EqualsAndHashCode;
import lombok.ToString;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE,setterVisibility = Visibility.NONE)
public class MsgPublishOutMsg {
@JsonProperty private Long offset;
@JsonProperty private String topic;
public MsgPublishOutMsg () {}

 public MsgPublishOutMsg offset(Long offset) {
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

     public MsgPublishOutMsg topic(String topic) {
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

    /**
     * Create an instance of MsgPublishOutMsg given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgPublishOutMsg
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgPublishOutMsg
     */
    public static MsgPublishOutMsg fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgPublishOutMsg.class);
    }

    /**
     * Convert an instance of MsgPublishOutMsg to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}