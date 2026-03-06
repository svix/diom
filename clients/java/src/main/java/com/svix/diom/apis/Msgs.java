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
import com.svix.diom.models.MsgNamespaceCreateIn;
import com.svix.diom.models.MsgNamespaceCreateOut;
import com.svix.diom.models.MsgNamespaceGetIn;
import com.svix.diom.models.MsgNamespaceGetOut;
import com.svix.diom.models.MsgPublishIn;
import com.svix.diom.models.MsgPublishOut;
import com.svix.diom.models.MsgQueueAckIn;
import com.svix.diom.models.MsgQueueAckOut;
import com.svix.diom.models.MsgQueueReceiveIn;
import com.svix.diom.models.MsgQueueReceiveOut;
import com.svix.diom.models.MsgStreamCommitIn;
import com.svix.diom.models.MsgStreamCommitOut;
import com.svix.diom.models.MsgStreamReceiveIn;
import com.svix.diom.models.MsgStreamReceiveOut;
import com.svix.diom.models.MsgTopicConfigureIn;
import com.svix.diom.models.MsgTopicConfigureOut;

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