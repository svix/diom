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
import com.svix.coyote.ByteArrayAsIntArraySerializer;
import com.svix.coyote.ByteArrayAsIntArrayDeserializer;
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
public class ListResponseAdminAccessPolicyOut {
    @JsonProperty private List<AdminAccessPolicyOut> data;
    @JsonProperty private String iterator;
    @JsonProperty("prev_iterator") private String prevIterator;
    @JsonProperty private Boolean done;
    public ListResponseAdminAccessPolicyOut() {}

    public ListResponseAdminAccessPolicyOut data(List<AdminAccessPolicyOut> data) {
        this.data = data;
        return this;
    }

    public ListResponseAdminAccessPolicyOut addDataItem(AdminAccessPolicyOut dataItem) {
        if (this.data == null) {
            this.data = new ArrayList<>();
        }
        this.data.add(dataItem);
        return this;
    }
    /**
    * Get data
    *
     * @return data
     */
    @javax.annotation.Nonnull
    public List<AdminAccessPolicyOut> getData() {
        return data;
    }

    public void setData(List<AdminAccessPolicyOut> data) {
        this.data = data;
    }

    public ListResponseAdminAccessPolicyOut iterator(String iterator) {
        this.iterator = iterator;
        return this;
    }

    /**
    * Get iterator
    *
     * @return iterator
     */
    @javax.annotation.Nullable
    public String getIterator() {
        return iterator;
    }

    public void setIterator(String iterator) {
        this.iterator = iterator;
    }

    public ListResponseAdminAccessPolicyOut prevIterator(String prevIterator) {
        this.prevIterator = prevIterator;
        return this;
    }

    /**
    * Get prevIterator
    *
     * @return prevIterator
     */
    @javax.annotation.Nullable
    public String getPrevIterator() {
        return prevIterator;
    }

    public void setPrevIterator(String prevIterator) {
        this.prevIterator = prevIterator;
    }

    public ListResponseAdminAccessPolicyOut done(Boolean done) {
        this.done = done;
        return this;
    }

    /**
    * Get done
    *
     * @return done
     */
    @javax.annotation.Nonnull
    public Boolean getDone() {
        return done;
    }

    public void setDone(Boolean done) {
        this.done = done;
    }

    /**
     * Create an instance of ListResponseAdminAccessPolicyOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ListResponseAdminAccessPolicyOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to ListResponseAdminAccessPolicyOut
     */
    public static ListResponseAdminAccessPolicyOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ListResponseAdminAccessPolicyOut.class);
    }

    /**
     * Convert an instance of ListResponseAdminAccessPolicyOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}