// this file is @generated
package com.svix.diom.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.diom.ApiException;
import com.svix.diom.HttpClient;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import com.svix.diom.models.ClusterForceSnapshotIn;
import com.svix.diom.models.ClusterForceSnapshotOut;
import com.svix.diom.models.ClusterInitializeIn;
import com.svix.diom.models.ClusterInitializeOut;
import com.svix.diom.models.ClusterRemoveNodeIn;
import com.svix.diom.models.ClusterRemoveNodeOut;
import com.svix.diom.models.ClusterStatusOut;

public class ClusterAdmin {
    private final HttpClient client;

    public ClusterAdmin(HttpClient client) {
        this.client = client;
    }

    /** Get information about the current cluster */
    public ClusterStatusOut status(
        
    ) throws IOException, ApiException {

        return this.client.executeRequest(
            "GET",
            "/api/v1.cluster-admin.status",
            null,
            null,
            ClusterStatusOut.class
        );
    }

    /**
* Initialize this node as the leader of a new cluster
* 
* This operation may only be performed against a node which has not been
* initialized and is not currently a member of a cluster.
*/
    public ClusterInitializeOut initialize(
        final ClusterInitializeIn clusterInitializeIn
    ) throws IOException, ApiException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.cluster-admin.initialize",
            null,
            clusterInitializeIn,
            ClusterInitializeOut.class
        );
    }

    /**
* Initialize this node as the leader of a new cluster
* 
* This operation may only be performed against a node which has not been
* initialized and is not currently a member of a cluster.
*/
    public ClusterInitializeOut initialize(
        
    ) throws IOException, ApiException {
        return this.initialize(
            new ClusterInitializeIn()
        );
    }

    /**
* Remove a node from the cluster.
* 
* This operation executes immediately and the node must be wiped and reset
* before it can safely be added to the cluster.
*/
    public ClusterRemoveNodeOut removeNode(
        final ClusterRemoveNodeIn clusterRemoveNodeIn
    ) throws IOException, ApiException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.cluster-admin.remove-node",
            null,
            clusterRemoveNodeIn,
            ClusterRemoveNodeOut.class
        );
    }

    /** Force the cluster to take a snapshot immediately */
    public ClusterForceSnapshotOut forceSnapshot(
        final ClusterForceSnapshotIn clusterForceSnapshotIn
    ) throws IOException, ApiException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.cluster-admin.force-snapshot",
            null,
            clusterForceSnapshotIn,
            ClusterForceSnapshotOut.class
        );
    }

    /** Force the cluster to take a snapshot immediately */
    public ClusterForceSnapshotOut forceSnapshot(
        
    ) throws IOException, ApiException {
        return this.forceSnapshot(
            new ClusterForceSnapshotIn()
        );
    }
}