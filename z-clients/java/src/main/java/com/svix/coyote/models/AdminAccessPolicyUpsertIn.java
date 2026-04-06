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
public class AdminAccessPolicyUpsertIn {
    @JsonProperty private String id;
    @JsonProperty private String description;
    @JsonProperty private List<AccessRule> rules;
    public AdminAccessPolicyUpsertIn() {}

    public AdminAccessPolicyUpsertIn id(String id) {
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

    public AdminAccessPolicyUpsertIn description(String description) {
        this.description = description;
        return this;
    }

    /**
    * Get description
    *
     * @return description
     */
    @javax.annotation.Nonnull
    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public AdminAccessPolicyUpsertIn rules(List<AccessRule> rules) {
        this.rules = rules;
        return this;
    }

    public AdminAccessPolicyUpsertIn addRulesItem(AccessRule rulesItem) {
        if (this.rules == null) {
            this.rules = new ArrayList<>();
        }
        this.rules.add(rulesItem);
        return this;
    }
    /**
    * Get rules
    *
     * @return rules
     */
    @javax.annotation.Nullable
    public List<AccessRule> getRules() {
        return rules;
    }

    public void setRules(List<AccessRule> rules) {
        this.rules = rules;
    }

    /**
     * Create an instance of AdminAccessPolicyUpsertIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AdminAccessPolicyUpsertIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AdminAccessPolicyUpsertIn
     */
    public static AdminAccessPolicyUpsertIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AdminAccessPolicyUpsertIn.class);
    }

    /**
     * Convert an instance of AdminAccessPolicyUpsertIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}