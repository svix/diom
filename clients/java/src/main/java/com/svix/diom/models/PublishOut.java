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
public class PublishOut {
@JsonProperty private List<PublishOutMsg> msgs;
public PublishOut () {}

 public PublishOut msgs(List<PublishOutMsg> msgs) {
        this.msgs = msgs;
        return this;
    }

     public PublishOut addMsgsItem(PublishOutMsg msgsItem) {
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
     public List<PublishOutMsg> getMsgs() {
        return msgs;
    }

     public void setMsgs(List<PublishOutMsg> msgs) {
        this.msgs = msgs;
    }

    /**
     * Create an instance of PublishOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PublishOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to PublishOut
     */
    public static PublishOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PublishOut.class);
    }

    /**
     * Convert an instance of PublishOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}