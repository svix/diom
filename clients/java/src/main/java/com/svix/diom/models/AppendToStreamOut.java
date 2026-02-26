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
@JsonAutoDetect(getterVisibility = Visibility.NONE,setterVisibility = Visibility.NONE)
public class AppendToStreamOut {
@JsonProperty("msg_ids") private List<Long> msgIds;
public AppendToStreamOut () {}

 public AppendToStreamOut msgIds(List<Long> msgIds) {
        this.msgIds = msgIds;
        return this;
    }

     public AppendToStreamOut addMsgIdsItem(Long msgIdsItem) {
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
     public List<Long> getMsgIds() {
        return msgIds;
    }

     public void setMsgIds(List<Long> msgIds) {
        this.msgIds = msgIds;
    }

    /**
     * Create an instance of AppendToStreamOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AppendToStreamOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to AppendToStreamOut
     */
    public static AppendToStreamOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AppendToStreamOut.class);
    }

    /**
     * Convert an instance of AppendToStreamOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}