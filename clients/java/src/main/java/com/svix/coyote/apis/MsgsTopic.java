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
import com.svix.coyote.models.TopicConfigureIn;
import com.svix.coyote.models.TopicConfigureOut;

public class MsgsTopic {
    private final HttpClient client;

    public MsgsTopic(HttpClient client) {
        this.client = client;
    }

    /**
* Configures the number of partitions for a topic.
* 
* Partition count can only be increased, never decreased. The default for a new topic is 1.
*/
    public TopicConfigureOut configure(
        final TopicConfigureIn topicConfigureIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/topic/configure");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            topicConfigureIn,
            TopicConfigureOut.class
            );
    }
}