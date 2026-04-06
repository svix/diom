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
public class NodeStatusOut {
    @JsonProperty("node_id") private String nodeId;
    @JsonProperty private String address;
    @JsonProperty private ServerState state;
    @JsonProperty("last_committed_log_index") private Long lastCommittedLogIndex;
    @JsonProperty("last_committed_term") private Long lastCommittedTerm;
    public NodeStatusOut() {}

    public NodeStatusOut nodeId(String nodeId) {
        this.nodeId = nodeId;
        return this;
    }

    /**
    * A unique ID representing this node.

This will never change unless the node is erased and reset
    *
     * @return nodeId
     */
    @javax.annotation.Nonnull
    public String getNodeId() {
        return nodeId;
    }

    public void setNodeId(String nodeId) {
        this.nodeId = nodeId;
    }

    public NodeStatusOut address(String address) {
        this.address = address;
        return this;
    }

    /**
    * The advertised inter-server (cluster) address of this node.
    *
     * @return address
     */
    @javax.annotation.Nonnull
    public String getAddress() {
        return address;
    }

    public void setAddress(String address) {
        this.address = address;
    }

    public NodeStatusOut state(ServerState state) {
        this.state = state;
        return this;
    }

    /**
    * The last known state of this node
    *
     * @return state
     */
    @javax.annotation.Nonnull
    public ServerState getState() {
        return state;
    }

    public void setState(ServerState state) {
        this.state = state;
    }

    public NodeStatusOut lastCommittedLogIndex(Long lastCommittedLogIndex) {
        this.lastCommittedLogIndex = lastCommittedLogIndex;
        return this;
    }

    /**
    * The index of the last log applied on this node
    *
     * @return lastCommittedLogIndex
     */
    @javax.annotation.Nullable
    public Long getLastCommittedLogIndex() {
        return lastCommittedLogIndex;
    }

    public void setLastCommittedLogIndex(Long lastCommittedLogIndex) {
        this.lastCommittedLogIndex = lastCommittedLogIndex;
    }

    public NodeStatusOut lastCommittedTerm(Long lastCommittedTerm) {
        this.lastCommittedTerm = lastCommittedTerm;
        return this;
    }

    /**
    * The raft term of the last committed leadership
    *
     * @return lastCommittedTerm
     */
    @javax.annotation.Nullable
    public Long getLastCommittedTerm() {
        return lastCommittedTerm;
    }

    public void setLastCommittedTerm(Long lastCommittedTerm) {
        this.lastCommittedTerm = lastCommittedTerm;
    }

    /**
     * Create an instance of NodeStatusOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of NodeStatusOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to NodeStatusOut
     */
    public static NodeStatusOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, NodeStatusOut.class);
    }

    /**
     * Convert an instance of NodeStatusOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}