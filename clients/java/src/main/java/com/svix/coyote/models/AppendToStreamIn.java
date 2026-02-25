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
public class AppendToStreamIn {
@JsonProperty private List<MsgIn> msgs;
@JsonProperty private String name;
public AppendToStreamIn () {}

 public AppendToStreamIn msgs(List<MsgIn> msgs) {
        this.msgs = msgs;
        return this;
    }

     public AppendToStreamIn addMsgsItem(MsgIn msgsItem) {
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
     public List<MsgIn> getMsgs() {
        return msgs;
    }

     public void setMsgs(List<MsgIn> msgs) {
        this.msgs = msgs;
    }

     public AppendToStreamIn name(String name) {
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

    /**
     * Create an instance of AppendToStreamIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AppendToStreamIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AppendToStreamIn
     */
    public static AppendToStreamIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AppendToStreamIn.class);
    }

    /**
     * Convert an instance of AppendToStreamIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}