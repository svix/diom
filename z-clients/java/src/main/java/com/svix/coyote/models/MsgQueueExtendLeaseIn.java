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
public class MsgQueueExtendLeaseIn {
    @JsonProperty private String namespace;
    @JsonProperty("msg_ids") private List<String> msgIds;
    @JsonProperty("lease_duration_ms") @JsonSerialize(using = DurationMsSerializer.class) @JsonDeserialize(using = DurationMsDeserializer.class) private Duration leaseDuration;
    public MsgQueueExtendLeaseIn() {}

    public MsgQueueExtendLeaseIn namespace(String namespace) {
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

    public MsgQueueExtendLeaseIn msgIds(List<String> msgIds) {
        this.msgIds = msgIds;
        return this;
    }

    public MsgQueueExtendLeaseIn addMsgIdsItem(String msgIdsItem) {
        if (this.msgIds == null) {
            this.msgIds = new ArrayList<>();
        }
        this.msgIds.add(msgIdsItem);
        return this;
    }
    /**
    * Get msgIds
    *
     * @return msgIds
     */
    @javax.annotation.Nonnull
    public List<String> getMsgIds() {
        return msgIds;
    }

    public void setMsgIds(List<String> msgIds) {
        this.msgIds = msgIds;
    }

    public MsgQueueExtendLeaseIn leaseDuration(Duration leaseDuration) {
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
}