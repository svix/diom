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
public class ClusterForceNodeDowngradeOut {
    @JsonProperty("initial_node_state") private ServerState initialNodeState;
    @JsonProperty("final_node_state") private ServerState finalNodeState;
    public ClusterForceNodeDowngradeOut() {}

    public ClusterForceNodeDowngradeOut initialNodeState(ServerState initialNodeState) {
        this.initialNodeState = initialNodeState;
        return this;
    }

    /**
    * Get initialNodeState
    *
     * @return initialNodeState
     */
    @javax.annotation.Nonnull
    public ServerState getInitialNodeState() {
        return initialNodeState;
    }

    public void setInitialNodeState(ServerState initialNodeState) {
        this.initialNodeState = initialNodeState;
    }

    public ClusterForceNodeDowngradeOut finalNodeState(ServerState finalNodeState) {
        this.finalNodeState = finalNodeState;
        return this;
    }

    /**
    * Get finalNodeState
    *
     * @return finalNodeState
     */
    @javax.annotation.Nonnull
    public ServerState getFinalNodeState() {
        return finalNodeState;
    }

    public void setFinalNodeState(ServerState finalNodeState) {
        this.finalNodeState = finalNodeState;
    }

    /**
     * Create an instance of ClusterForceNodeDowngradeOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClusterForceNodeDowngradeOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to ClusterForceNodeDowngradeOut
     */
    public static ClusterForceNodeDowngradeOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClusterForceNodeDowngradeOut.class);
    }

    /**
     * Convert an instance of ClusterForceNodeDowngradeOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}