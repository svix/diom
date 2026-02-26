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
public class CacheSetIn {
@JsonProperty private String key;
@JsonProperty private Long ttl;
@JsonProperty private List<Byte> value;
public CacheSetIn () {}

 public CacheSetIn key(String key) {
        this.key = key;
        return this;
    }

    /**
    * Get key
    *
     * @return key
     */
    @javax.annotation.Nonnull
     public String getKey() {
        return key;
    }

     public void setKey(String key) {
        this.key = key;
    }

     public CacheSetIn ttl(Long ttl) {
        this.ttl = ttl;
        return this;
    }

    /**
    * Time to live in milliseconds
    *
     * @return ttl
     */
    @javax.annotation.Nonnull
     public Long getTtl() {
        return ttl;
    }

     public void setTtl(Long ttl) {
        this.ttl = ttl;
    }

     public CacheSetIn value(List<Byte> value) {
        this.value = value;
        return this;
    }

     public CacheSetIn addValueItem(Byte valueItem) {
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
     * Create an instance of CacheSetIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CacheSetIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to CacheSetIn
     */
    public static CacheSetIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CacheSetIn.class);
    }

    /**
     * Convert an instance of CacheSetIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}