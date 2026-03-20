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
public class MsgStreamReceiveIn {
    @JsonProperty private String namespace;
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty("batch_size") private Short batchSize;
    @JsonProperty("lease_duration_millis") private Long leaseDurationMillis;
    public MsgStreamReceiveIn() {}

    public MsgStreamReceiveIn namespace(String namespace) {
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

    public MsgStreamReceiveIn batchSize(Short batchSize) {
        this.batchSize = batchSize;
        return this;
    }

    /**
    * Get batchSize
    *
     * @return batchSize
     */
    @javax.annotation.Nullable
    public Short getBatchSize() {
        return batchSize;
    }

    public void setBatchSize(Short batchSize) {
        this.batchSize = batchSize;
    }

    public MsgStreamReceiveIn leaseDurationMillis(Long leaseDurationMillis) {
        this.leaseDurationMillis = leaseDurationMillis;
        return this;
    }

    /**
    * Get leaseDurationMillis
    *
     * @return leaseDurationMillis
     */
    @javax.annotation.Nullable
    public Long getLeaseDurationMillis() {
        return leaseDurationMillis;
    }

    public void setLeaseDurationMillis(Long leaseDurationMillis) {
        this.leaseDurationMillis = leaseDurationMillis;
    }
}