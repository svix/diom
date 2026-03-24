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
import lombok.Getter;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.coyote.models.ClusterRemoveNodeIn;
import com.svix.coyote.models.ClusterRemoveNodeOut;

public class Admin {
    private final HttpClient client;

    public Admin(HttpClient client) {
        this.client = client;
    }

    /**
* Remove a node from the cluster.
* 
* This operation executes immediately and the node must be wiped and reset
* before it can safely be added to the cluster.
*/
    public ClusterRemoveNodeOut clusterRemoveNode(
        final ClusterRemoveNodeIn clusterRemoveNodeIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/admin/cluster/remove-node");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            clusterRemoveNodeIn,
            ClusterRemoveNodeOut.class
        );
    }
}