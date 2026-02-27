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
public class PublishOutMsg {
@JsonProperty private Long offset;
@JsonProperty private Short partition;
public PublishOutMsg () {}

 public PublishOutMsg offset(Long offset) {
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

     public PublishOutMsg partition(Short partition) {
        this.partition = partition;
        return this;
    }

    /**
    * Get partition
    *
     * @return partition
     */
    @javax.annotation.Nonnull
     public Short getPartition() {
        return partition;
    }

     public void setPartition(Short partition) {
        this.partition = partition;
    }

    /**
     * Create an instance of PublishOutMsg given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PublishOutMsg
     * @throws JsonProcessingException if the JSON string is invalid with respect to PublishOutMsg
     */
    public static PublishOutMsg fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PublishOutMsg.class);
    }

    /**
     * Convert an instance of PublishOutMsg to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}