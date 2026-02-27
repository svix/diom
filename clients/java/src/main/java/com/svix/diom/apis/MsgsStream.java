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
import com.svix.diom.models.StreamCommitIn;
import com.svix.diom.models.StreamCommitOut;
import com.svix.diom.models.StreamReceiveIn;
import com.svix.diom.models.StreamReceiveOut;

public class MsgsStream {
    private final HttpClient client;

    public MsgsStream(HttpClient client) {
        this.client = client;
    }

    /**
* Receives messages from a topic using a consumer group.
* 
* Each consumer in the group reads from all partitions. Messages are locked by leases for the
* specified duration to prevent duplicate delivery within the same consumer group.
*/
    public StreamReceiveOut receive(
        final StreamReceiveIn streamReceiveIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/stream/receive");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            streamReceiveIn,
            StreamReceiveOut.class
            );
    }

    /**
* Commits an offset for a consumer group on a specific partition.
* 
* The topic must be a partition-level topic (e.g. `my-topic~3`). The offset is the last
* successfully processed offset; future receives will start after it.
*/
    public StreamCommitOut commit(
        final StreamCommitIn streamCommitIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/stream/commit");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            streamCommitIn,
            StreamCommitOut.class
            );
    }
}