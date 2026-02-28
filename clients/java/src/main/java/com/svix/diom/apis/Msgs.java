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
import lombok.Getter;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.CreateNamespaceIn;
import com.svix.diom.models.CreateNamespaceOut;
import com.svix.diom.models.GetNamespaceIn;
import com.svix.diom.models.GetNamespaceOut;
import com.svix.diom.models.PublishIn;
import com.svix.diom.models.PublishOut;
import com.svix.diom.models.StreamCommitIn;
import com.svix.diom.models.StreamCommitOut;
import com.svix.diom.models.StreamReceiveIn;
import com.svix.diom.models.StreamReceiveOut;
import com.svix.diom.models.TopicConfigureIn;
import com.svix.diom.models.TopicConfigureOut;

public class Msgs {
    private final HttpClient client;

    public Msgs(HttpClient client) {
        this.client = client;
    }

    /** Publishes messages to a topic within a namespace. */
    public PublishOut publish(
        final PublishIn publishIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/publish");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            publishIn,
            PublishOut.class
            );
    }
}