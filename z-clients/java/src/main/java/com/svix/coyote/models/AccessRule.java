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
public class AccessRule {
    @JsonProperty private AccessRuleEffect effect;
    @JsonProperty private String resource;
    @JsonProperty private List<String> actions;
    public AccessRule() {}

    public AccessRule effect(AccessRuleEffect effect) {
        this.effect = effect;
        return this;
    }

    /**
    * Get effect
    *
     * @return effect
     */
    @javax.annotation.Nonnull
    public AccessRuleEffect getEffect() {
        return effect;
    }

    public void setEffect(AccessRuleEffect effect) {
        this.effect = effect;
    }

    public AccessRule resource(String resource) {
        this.resource = resource;
        return this;
    }

    /**
    * Get resource
    *
     * @return resource
     */
    @javax.annotation.Nonnull
    public String getResource() {
        return resource;
    }

    public void setResource(String resource) {
        this.resource = resource;
    }

    public AccessRule actions(List<String> actions) {
        this.actions = actions;
        return this;
    }

    public AccessRule addActionsItem(String actionsItem) {
        if (this.actions == null) {
            this.actions = new ArrayList<>();
        }
        this.actions.add(actionsItem);
        return this;
    }
    /**
    * Get actions
    *
     * @return actions
     */
    @javax.annotation.Nonnull
    public List<String> getActions() {
        return actions;
    }

    public void setActions(List<String> actions) {
        this.actions = actions;
    }

    /**
     * Create an instance of AccessRule given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AccessRule
     * @throws JsonProcessingException if the JSON string is invalid with respect to AccessRule
     */
    public static AccessRule fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AccessRule.class);
    }

    /**
     * Convert an instance of AccessRule to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}