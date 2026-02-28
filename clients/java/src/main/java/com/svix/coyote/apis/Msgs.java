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
import com.svix.coyote.models.MsgNamespaceCreateIn;
import com.svix.coyote.models.MsgNamespaceCreateOut;
import com.svix.coyote.models.MsgNamespaceGetIn;
import com.svix.coyote.models.MsgNamespaceGetOut;
import com.svix.coyote.models.MsgPublishIn;
import com.svix.coyote.models.MsgPublishOut;
import com.svix.coyote.models.MsgStreamCommitIn;
import com.svix.coyote.models.MsgStreamCommitOut;
import com.svix.coyote.models.MsgStreamReceiveIn;
import com.svix.coyote.models.MsgStreamReceiveOut;
import com.svix.coyote.models.MsgTopicConfigureIn;
import com.svix.coyote.models.MsgTopicConfigureOut;

public class Msgs {
    private final HttpClient client;

    public Msgs(HttpClient client) {
        this.client = client;
    }

    /** Publishes messages to a topic within a namespace. */
    public MsgPublishOut publish(
        final MsgPublishIn msgPublishIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/publish");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgPublishIn,
            MsgPublishOut.class
            );
    }
}