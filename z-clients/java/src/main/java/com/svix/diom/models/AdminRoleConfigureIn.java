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
import com.svix.diom.UnixTimestampMsSerializer;
import com.svix.diom.UnixTimestampMsDeserializer;
import com.svix.diom.Utils;
import java.time.Duration;
import java.time.Instant;
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
public class AdminRoleConfigureIn {
    @JsonProperty private String id;
    @JsonProperty private String description;
    @JsonProperty private List<AccessRule> rules;
    @JsonProperty private List<String> policies;
    @JsonProperty private Map<String,String> context;
    public AdminRoleConfigureIn() {}

    public AdminRoleConfigureIn id(String id) {
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

    public AdminRoleConfigureIn description(String description) {
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

    public AdminRoleConfigureIn rules(List<AccessRule> rules) {
        this.rules = rules;
        return this;
    }

    public AdminRoleConfigureIn addRulesItem(AccessRule rulesItem) {
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

    public AdminRoleConfigureIn policies(List<String> policies) {
        this.policies = policies;
        return this;
    }

    public AdminRoleConfigureIn addPoliciesItem(String policiesItem) {
        if (this.policies == null) {
            this.policies = new ArrayList<>();
        }
        this.policies.add(policiesItem);
        return this;
    }
    /**
    * Get policies
    *
     * @return policies
     */
    @javax.annotation.Nullable
    public List<String> getPolicies() {
        return policies;
    }

    public void setPolicies(List<String> policies) {
        this.policies = policies;
    }

    public AdminRoleConfigureIn context(Map<String,String> context) {
        this.context = context;
        return this;
    }

    public AdminRoleConfigureIn putContextItem(String key, String contextItem) {
        if (this.context == null) {
            this.context = new HashMap<>();
        }
        this.context.put(key, contextItem);
        return this;
    }
    /**
    * Get context
    *
     * @return context
     */
    @javax.annotation.Nullable
    public Map<String,String> getContext() {
        return context;
    }

    public void setContext(Map<String,String> context) {
        this.context = context;
    }

    /**
     * Create an instance of AdminRoleConfigureIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AdminRoleConfigureIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AdminRoleConfigureIn
     */
    public static AdminRoleConfigureIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AdminRoleConfigureIn.class);
    }

    /**
     * Convert an instance of AdminRoleConfigureIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}