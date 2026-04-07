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
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.svix.diom.DurationMsSerializer;
import com.svix.diom.DurationMsDeserializer;
import com.svix.diom.Utils;
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
public class AdminAccessPolicyUpsertOut {
    @JsonProperty private String id;
    @JsonProperty private Long created;
    @JsonProperty private Long updated;
    public AdminAccessPolicyUpsertOut() {}

    public AdminAccessPolicyUpsertOut id(String id) {
        this.id = id;
        return this;
    }

    /**
    * Get id
    *
     * @return id
     */
    @javax.annotation.Nonnull
    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public AdminAccessPolicyUpsertOut created(Long created) {
        this.created = created;
        return this;
    }

    /**
    * Get created
    *
     * @return created
     */
    @javax.annotation.Nonnull
    public Long getCreated() {
        return created;
    }

    public void setCreated(Long created) {
        this.created = created;
    }

    public AdminAccessPolicyUpsertOut updated(Long updated) {
        this.updated = updated;
        return this;
    }

    /**
    * Get updated
    *
     * @return updated
     */
    @javax.annotation.Nonnull
    public Long getUpdated() {
        return updated;
    }

    public void setUpdated(Long updated) {
        this.updated = updated;
    }

    /**
     * Create an instance of AdminAccessPolicyUpsertOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AdminAccessPolicyUpsertOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to AdminAccessPolicyUpsertOut
     */
    public static AdminAccessPolicyUpsertOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AdminAccessPolicyUpsertOut.class);
    }

    /**
     * Convert an instance of AdminAccessPolicyUpsertOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}