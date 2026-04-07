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
import com.svix.diom.ByteArrayAsIntArraySerializer;
import com.svix.diom.ByteArrayAsIntArrayDeserializer;
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
public class AdminRoleOut {
    @JsonProperty private String id;
    @JsonProperty private String description;
    @JsonProperty private List<AccessRule> rules;
    @JsonProperty private List<String> policies;
    @JsonProperty private Map<String,String> context;
    @JsonProperty private OffsetDateTime created;
    @JsonProperty private OffsetDateTime updated;
    public AdminRoleOut() {}

    public AdminRoleOut id(String id) {
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

    public AdminRoleOut description(String description) {
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

    public AdminRoleOut rules(List<AccessRule> rules) {
        this.rules = rules;
        return this;
    }

    public AdminRoleOut addRulesItem(AccessRule rulesItem) {
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
    @javax.annotation.Nonnull
    public List<AccessRule> getRules() {
        return rules;
    }

    public void setRules(List<AccessRule> rules) {
        this.rules = rules;
    }

    public AdminRoleOut policies(List<String> policies) {
        this.policies = policies;
        return this;
    }

    public AdminRoleOut addPoliciesItem(String policiesItem) {
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
    @javax.annotation.Nonnull
    public List<String> getPolicies() {
        return policies;
    }

    public void setPolicies(List<String> policies) {
        this.policies = policies;
    }

    public AdminRoleOut context(Map<String,String> context) {
        this.context = context;
        return this;
    }

    public AdminRoleOut putContextItem(String key, String contextItem) {
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
    @javax.annotation.Nonnull
    public Map<String,String> getContext() {
        return context;
    }

    public void setContext(Map<String,String> context) {
        this.context = context;
    }

    public AdminRoleOut created(OffsetDateTime created) {
        this.created = created;
        return this;
    }

    /**
    * Get created
    *
     * @return created
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getCreated() {
        return created;
    }

    public void setCreated(OffsetDateTime created) {
        this.created = created;
    }

    public AdminRoleOut updated(OffsetDateTime updated) {
        this.updated = updated;
        return this;
    }

    /**
    * Get updated
    *
     * @return updated
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getUpdated() {
        return updated;
    }

    public void setUpdated(OffsetDateTime updated) {
        this.updated = updated;
    }

    /**
     * Create an instance of AdminRoleOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AdminRoleOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to AdminRoleOut
     */
    public static AdminRoleOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AdminRoleOut.class);
    }

    /**
     * Convert an instance of AdminRoleOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}