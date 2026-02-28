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
import com.svix.coyote.models.CreateNamespaceIn;
import com.svix.coyote.models.CreateNamespaceOut;
import com.svix.coyote.models.GetNamespaceIn;
import com.svix.coyote.models.GetNamespaceOut;
import com.svix.coyote.models.PublishIn;
import com.svix.coyote.models.PublishOut;
import com.svix.coyote.models.StreamCommitIn;
import com.svix.coyote.models.StreamCommitOut;
import com.svix.coyote.models.StreamReceiveIn;
import com.svix.coyote.models.StreamReceiveOut;
import com.svix.coyote.models.TopicConfigureIn;
import com.svix.coyote.models.TopicConfigureOut;

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