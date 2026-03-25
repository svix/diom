// this file is @generated
package com.svix.diom.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.diom.ApiException;
import com.svix.diom.HttpClient;
import com.svix.diom.Utils;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.ClusterRemoveNodeIn;
import com.svix.diom.models.ClusterRemoveNodeOut;
import com.svix.diom.models.ClusterStatusOut;

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
}