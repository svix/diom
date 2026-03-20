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
import com.svix.diom.Utils;
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
public class MsgQueueAckIn {
    @JsonProperty private String namespace;
    @JsonProperty private String topic;
    @JsonProperty("consumer_group") private String consumerGroup;
    @JsonProperty("msg_ids") private List<String> msgIds;
    public MsgQueueAckIn() {}

    public MsgQueueAckIn namespace(String namespace) {
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

    public MsgQueueAckIn msgIds(List<String> msgIds) {
        this.msgIds = msgIds;
        return this;
    }

    public MsgQueueAckIn addMsgIdsItem(String msgIdsItem) {
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
}