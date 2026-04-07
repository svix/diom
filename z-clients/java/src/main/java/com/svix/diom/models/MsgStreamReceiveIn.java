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
import com.svix.diom.ByteArrayAsIntArraySerializer;
import com.svix.diom.ByteArrayAsIntArrayDeserializer;
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
    @JsonProperty("batch_size") private Short batchSize;
    @JsonProperty("lease_duration_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration leaseDuration;
    @JsonProperty("default_starting_position") private SeekPosition defaultStartingPosition;
    @JsonProperty("batch_wait_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration batchWait;
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

    public MsgStreamReceiveIn leaseDuration(Duration leaseDuration) {
        this.leaseDuration = leaseDuration;
        return this;
    }

    /**
    * Get leaseDuration
    *
     * @return leaseDuration
     */
    @javax.annotation.Nullable
    public Duration getLeaseDuration() {
        return leaseDuration;
    }

    public void setLeaseDuration(Duration leaseDuration) {
        this.leaseDuration = leaseDuration;
    }

    public MsgStreamReceiveIn defaultStartingPosition(SeekPosition defaultStartingPosition) {
        this.defaultStartingPosition = defaultStartingPosition;
        return this;
    }

    /**
    * Get defaultStartingPosition
    *
     * @return defaultStartingPosition
     */
    @javax.annotation.Nullable
    public SeekPosition getDefaultStartingPosition() {
        return defaultStartingPosition;
    }

    public void setDefaultStartingPosition(SeekPosition defaultStartingPosition) {
        this.defaultStartingPosition = defaultStartingPosition;
    }

    public MsgStreamReceiveIn batchWait(Duration batchWait) {
        this.batchWait = batchWait;
        return this;
    }

    /**
    * Maximum time (in milliseconds) to wait for messages before returning.
    *
     * @return batchWait
     */
    @javax.annotation.Nullable
    public Duration getBatchWait() {
        return batchWait;
    }

    public void setBatchWait(Duration batchWait) {
        this.batchWait = batchWait;
    }
}