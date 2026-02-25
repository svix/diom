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
public class MsgOut {
@JsonProperty private Map<String,String> headers;
@JsonProperty private Long id;
@JsonProperty private List<Byte> payload;
@JsonProperty private OffsetDateTime timestamp;
public MsgOut () {}

 public MsgOut headers(Map<String,String> headers) {
        this.headers = headers;
        return this;
    }

     public MsgOut putHeadersItem(String key, String headersItem) {
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
    @javax.annotation.Nonnull
     public Map<String,String> getHeaders() {
        return headers;
    }

     public void setHeaders(Map<String,String> headers) {
        this.headers = headers;
    }

     public MsgOut id(Long id) {
        this.id = id;
        return this;
    }

    /**
    * Get id
    *
     * @return id
     */
    @javax.annotation.Nonnull
     public Long getId() {
        return id;
    }

     public void setId(Long id) {
        this.id = id;
    }

     public MsgOut payload(List<Byte> payload) {
        this.payload = payload;
        return this;
    }

     public MsgOut addPayloadItem(Byte payloadItem) {
        if (this.payload == null) {
            this.payload = new ArrayList<>();
        }
        this.payload.add(payloadItem);
        return this;
    }
    /**
    * Get payload
    *
     * @return payload
     */
    @javax.annotation.Nonnull
     public List<Byte> getPayload() {
        return payload;
    }

     public void setPayload(List<Byte> payload) {
        this.payload = payload;
    }

     public MsgOut timestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
        return this;
    }

    /**
    * Get timestamp
    *
     * @return timestamp
     */
    @javax.annotation.Nonnull
     public OffsetDateTime getTimestamp() {
        return timestamp;
    }

     public void setTimestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
    }

    /**
     * Create an instance of MsgOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgOut
     */
    public static MsgOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgOut.class);
    }

    /**
     * Convert an instance of MsgOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}