// this file is @generated
package com.svix.coyote.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.coyote.ApiException;
import com.svix.coyote.HttpClient;
import com.svix.coyote.Utils;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.coyote.models.ClusterForceSnapshotIn;
import com.svix.coyote.models.ClusterForceSnapshotOut;
import com.svix.coyote.models.ClusterInitializeIn;
import com.svix.coyote.models.ClusterInitializeOut;
import com.svix.coyote.models.ClusterRemoveNodeIn;
import com.svix.coyote.models.ClusterRemoveNodeOut;
import com.svix.coyote.models.ClusterStatusOut;

public class AdminCluster {
    private final HttpClient client;

    public AdminCluster(HttpClient client) {
        this.client = client;
    }

    /** Get information about the current cluster */
    public ClusterStatusOut status(
        
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.cluster.status");

        return this.client.executeRequest(
            "GET",
            url.build(),
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
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.cluster.initialize");

        return this.client.executeRequest(
            "POST",
            url.build(),
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
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.cluster.remove-node");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            clusterRemoveNodeIn,
            ClusterRemoveNodeOut.class
        );
    }

    /** Force the cluster to take a snapshot immediately */
    public ClusterForceSnapshotOut forceSnapshot(
        final ClusterForceSnapshotIn clusterForceSnapshotIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.cluster.force-snapshot");

        return this.client.executeRequest(
            "POST",
            url.build(),
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