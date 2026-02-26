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
public class Ack {
@JsonProperty("consumer_group") private String consumerGroup;
@JsonProperty("msg_id") private Long msgId;
@JsonProperty private String name;
public Ack () {}

 public Ack consumerGroup(String consumerGroup) {
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

     public Ack msgId(Long msgId) {
        this.msgId = msgId;
        return this;
    }

    /**
    * Get msgId
    *
     * @return msgId
     */
    @javax.annotation.Nonnull
     public Long getMsgId() {
        return msgId;
    }

     public void setMsgId(Long msgId) {
        this.msgId = msgId;
    }

     public Ack name(String name) {
        this.name = name;
        return this;
    }

    /**
    * Get name
    *
     * @return name
     */
    @javax.annotation.Nonnull
     public String getName() {
        return name;
    }

     public void setName(String name) {
        this.name = name;
    }

    /**
     * Create an instance of Ack given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of Ack
     * @throws JsonProcessingException if the JSON string is invalid with respect to Ack
     */
    public static Ack fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, Ack.class);
    }

    /**
     * Convert an instance of Ack to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}