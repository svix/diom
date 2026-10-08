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
public class MsgTopicPartitionDescribeOut {
    @JsonProperty("partition_id") private Short partitionId;
    @JsonProperty("high_water_mark") private Long highWaterMark;
    public MsgTopicPartitionDescribeOut() {}

    public MsgTopicPartitionDescribeOut partitionId(Short partitionId) {
        this.partitionId = partitionId;
        return this;
    }

    /**
    * Get partitionId
    *
     * @return partitionId
     */
    @javax.annotation.Nonnull
    public Short getPartitionId() {
        return partitionId;
    }

    public void setPartitionId(Short partitionId) {
        this.partitionId = partitionId;
    }

    public MsgTopicPartitionDescribeOut highWaterMark(Long highWaterMark) {
        this.highWaterMark = highWaterMark;
        return this;
    }

    /**
    * The next offset to be committed to this partition
    *
     * @return highWaterMark
     */
    @javax.annotation.Nonnull
    public Long getHighWaterMark() {
        return highWaterMark;
    }

    public void setHighWaterMark(Long highWaterMark) {
        this.highWaterMark = highWaterMark;
    }

    /**
     * Create an instance of MsgTopicPartitionDescribeOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgTopicPartitionDescribeOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgTopicPartitionDescribeOut
     */
    public static MsgTopicPartitionDescribeOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgTopicPartitionDescribeOut.class);
    }

    /**
     * Convert an instance of MsgTopicPartitionDescribeOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}