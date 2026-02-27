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
public class MsgIn2 {
@JsonProperty private Map<String,String> headers;
@JsonProperty private String key;
@JsonProperty private List<Byte> value;
public MsgIn2 () {}

 public MsgIn2 headers(Map<String,String> headers) {
        this.headers = headers;
        return this;
    }

     public MsgIn2 putHeadersItem(String key, String headersItem) {
        if (this.headers == null) {
            this.headers = new HashMap<>();
        }
        this.headers.put(key, headersItem);
        return this;
    }
    /**
    * Get headers
    *
     * @return headers
     */
    @javax.annotation.Nullable
     public Map<String,String> getHeaders() {
        return headers;
    }

     public void setHeaders(Map<String,String> headers) {
        this.headers = headers;
    }

     public MsgIn2 key(String key) {
        this.key = key;
        return this;
    }

    /**
    * Optional partition key. Messages with the same key are routed to the same partition.
    *
     * @return key
     */
    @javax.annotation.Nullable
     public String getKey() {
        return key;
    }

     public void setKey(String key) {
        this.key = key;
    }

     public MsgIn2 value(List<Byte> value) {
        this.value = value;
        return this;
    }

     public MsgIn2 addValueItem(Byte valueItem) {
        if (this.value == null) {
            this.value = new ArrayList<>();
        }
        this.value.add(valueItem);
        return this;
    }
    /**
    * Get value
    *
     * @return value
     */
    @javax.annotation.Nonnull
     public List<Byte> getValue() {
        return value;
    }

     public void setValue(List<Byte> value) {
        this.value = value;
    }

    /**
     * Create an instance of MsgIn2 given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgIn2
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgIn2
     */
    public static MsgIn2 fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgIn2.class);
    }

    /**
     * Convert an instance of MsgIn2 to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}