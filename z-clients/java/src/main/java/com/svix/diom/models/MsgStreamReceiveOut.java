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
public class MsgStreamReceiveOut {
    @JsonProperty private List<StreamMsgOut> msgs;
    public MsgStreamReceiveOut() {}

    public MsgStreamReceiveOut msgs(List<StreamMsgOut> msgs) {
        this.msgs = msgs;
        return this;
    }

    public MsgStreamReceiveOut addMsgsItem(StreamMsgOut msgsItem) {
        if (this.msgs == null) {
            this.msgs = new ArrayList<>();
        }
        this.msgs.add(msgsItem);
        return this;
    }
    /**
    * Get msgs
    *
     * @return msgs
     */
    @javax.annotation.Nonnull
    public List<StreamMsgOut> getMsgs() {
        return msgs;
    }

    public void setMsgs(List<StreamMsgOut> msgs) {
        this.msgs = msgs;
    }

    /**
     * Create an instance of MsgStreamReceiveOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgStreamReceiveOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgStreamReceiveOut
     */
    public static MsgStreamReceiveOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgStreamReceiveOut.class);
    }

    /**
     * Convert an instance of MsgStreamReceiveOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}