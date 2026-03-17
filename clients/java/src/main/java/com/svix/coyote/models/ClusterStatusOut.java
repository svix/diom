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
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ClusterStatusOut {
    @JsonProperty("cluster_id") private String clusterId;
    @JsonProperty("cluster_name") private String clusterName;
    @JsonProperty("this_node_id") private String thisNodeId;
    @JsonProperty("this_node_state") private ServerState thisNodeState;
    @JsonProperty("this_node_last_committed_timestamp") private OffsetDateTime thisNodeLastCommittedTimestamp;
    @JsonProperty private List<NodeStatusOut> nodes;
    public ClusterStatusOut () {}

    public ClusterStatusOut clusterId(String clusterId) {
        this.clusterId = clusterId;
        return this;
    }

    /**
    * The unique ID of this cluster.pub(crate)

This value is populated on cluster initialization and will never change.
    *
     * @return clusterId
     */
    @javax.annotation.Nullable
    public String getClusterId() {
        return clusterId;
    }

    public void setClusterId(String clusterId) {
        this.clusterId = clusterId;
    }

    public ClusterStatusOut clusterName(String clusterName) {
        this.clusterName = clusterName;
        return this;
    }

    /**
    * The name of this cluster (as defined in the config)

This value is not replicated and should only be used for debugging.
    *
     * @return clusterName
     */
    @javax.annotation.Nullable
    public String getClusterName() {
        return clusterName;
    }

    public void setClusterName(String clusterName) {
        this.clusterName = clusterName;
    }

    public ClusterStatusOut thisNodeId(String thisNodeId) {
        this.thisNodeId = thisNodeId;
        return this;
    }

    /**
    * The unique ID of the node servicing this request
    *
     * @return thisNodeId
     */
    @javax.annotation.Nonnull
    public String getThisNodeId() {
        return thisNodeId;
    }

    public void setThisNodeId(String thisNodeId) {
        this.thisNodeId = thisNodeId;
    }

    public ClusterStatusOut thisNodeState(ServerState thisNodeState) {
        this.thisNodeState = thisNodeState;
        return this;
    }

    /**
    * The cluster state of the node servicing this request
    *
     * @return thisNodeState
     */
    @javax.annotation.Nonnull
    public ServerState getThisNodeState() {
        return thisNodeState;
    }

    public void setThisNodeState(ServerState thisNodeState) {
        this.thisNodeState = thisNodeState;
    }

    public ClusterStatusOut thisNodeLastCommittedTimestamp(OffsetDateTime thisNodeLastCommittedTimestamp) {
        this.thisNodeLastCommittedTimestamp = thisNodeLastCommittedTimestamp;
        return this;
    }

    /**
    * The timestamp of the last transaction committed on this node
    *
     * @return thisNodeLastCommittedTimestamp
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getThisNodeLastCommittedTimestamp() {
        return thisNodeLastCommittedTimestamp;
    }

    public void setThisNodeLastCommittedTimestamp(OffsetDateTime thisNodeLastCommittedTimestamp) {
        this.thisNodeLastCommittedTimestamp = thisNodeLastCommittedTimestamp;
    }

    public ClusterStatusOut nodes(List<NodeStatusOut> nodes) {
        this.nodes = nodes;
        return this;
    }

    public ClusterStatusOut addNodesItem(NodeStatusOut nodesItem) {
        if (this.nodes == null) {
            this.nodes = new ArrayList<>();
        }
        this.nodes.add(nodesItem);
        return this;
    }
    /**
    * A list of all nodes known to be in the cluster
    *
     * @return nodes
     */
    @javax.annotation.Nonnull
    public List<NodeStatusOut> getNodes() {
        return nodes;
    }

    public void setNodes(List<NodeStatusOut> nodes) {
        this.nodes = nodes;
    }
    /**
     * Create an instance of ClusterStatusOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClusterStatusOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to ClusterStatusOut
     */
    public static ClusterStatusOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClusterStatusOut.class);
    }

    /**
     * Convert an instance of ClusterStatusOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}