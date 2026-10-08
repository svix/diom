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
public class MsgTopicDescribeOut {
    @JsonProperty private String id;
    @JsonProperty private String name;
    @JsonProperty private List<MsgTopicPartitionDescribeOut> partitions;
    public MsgTopicDescribeOut() {}

    public MsgTopicDescribeOut id(String id) {
        this.id = id;
        return this;
    }

    /**
    * The unique internal ID of this topic

This can useful for debugging
    *
     * @return id
     */
    @javax.annotation.Nonnull
    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public MsgTopicDescribeOut name(String name) {
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

    public MsgTopicDescribeOut partitions(List<MsgTopicPartitionDescribeOut> partitions) {
        this.partitions = partitions;
        return this;
    }

    public MsgTopicDescribeOut addPartitionsItem(MsgTopicPartitionDescribeOut partitionsItem) {
        if (this.partitions == null) {
            this.partitions = new ArrayList<>();
        }
        this.partitions.add(partitionsItem);
        return this;
    }
    /**
    * Get partitions
    *
     * @return partitions
     */
    @javax.annotation.Nonnull
    public List<MsgTopicPartitionDescribeOut> getPartitions() {
        return partitions;
    }

    public void setPartitions(List<MsgTopicPartitionDescribeOut> partitions) {
        this.partitions = partitions;
    }

    /**
     * Create an instance of MsgTopicDescribeOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgTopicDescribeOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgTopicDescribeOut
     */
    public static MsgTopicDescribeOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgTopicDescribeOut.class);
    }

    /**
     * Convert an instance of MsgTopicDescribeOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}