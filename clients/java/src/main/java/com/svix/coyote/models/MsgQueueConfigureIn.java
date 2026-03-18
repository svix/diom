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
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MsgQueueConfigureIn {
    @JsonProperty private String namespace;
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty("retry_schedule") private List<Long> retrySchedule;
    @JsonProperty("dlq_topic") private String dlqTopic;
    public MsgQueueConfigureIn() {}

    public MsgQueueConfigureIn namespace(String namespace) {
        this.namespace = namespace;
        return this;
    }

    /**
    * Get namespace
    *
     * @return namespace
     */
    @javax.annotation.Nullable
    public String getNamespace() {
        return namespace;
    }

    public void setNamespace(String namespace) {
        this.namespace = namespace;
    }

    public MsgQueueConfigureIn topic(String topic) {
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

    public MsgQueueConfigureIn consumerGroup(String consumerGroup) {
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

    public MsgQueueConfigureIn retrySchedule(List<Long> retrySchedule) {
        this.retrySchedule = retrySchedule;
        return this;
    }

    public MsgQueueConfigureIn addRetryScheduleItem(Long retryScheduleItem) {
        if (this.retrySchedule == null) {
            this.retrySchedule = new ArrayList<>();
        }
        this.retrySchedule.add(retryScheduleItem);
        return this;
    }
    /**
    * Get retrySchedule
    *
     * @return retrySchedule
     */
    @javax.annotation.Nullable
    public List<Long> getRetrySchedule() {
        return retrySchedule;
    }

    public void setRetrySchedule(List<Long> retrySchedule) {
        this.retrySchedule = retrySchedule;
    }

    public MsgQueueConfigureIn dlqTopic(String dlqTopic) {
        this.dlqTopic = dlqTopic;
        return this;
    }

    /**
    * Get dlqTopic
    *
     * @return dlqTopic
     */
    @javax.annotation.Nullable
    public String getDlqTopic() {
        return dlqTopic;
    }

    public void setDlqTopic(String dlqTopic) {
        this.dlqTopic = dlqTopic;
    }

    /**
     * Create an instance of MsgQueueConfigureIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgQueueConfigureIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgQueueConfigureIn
     */
    public static MsgQueueConfigureIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgQueueConfigureIn.class);
    }

    /**
     * Convert an instance of MsgQueueConfigureIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}