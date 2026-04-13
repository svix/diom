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
import java.util.List;
import java.util.Map;
import java.util.Set;
import lombok.Getter;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.MsgPublishIn;
import com.svix.diom.models.MsgPublishOut;
import com.svix.diom.models.MsgPublishIn_;

public class Msgs {
    private final HttpClient client;

    public Msgs(HttpClient client) {
        this.client = client;
    }

    /** Publishes messages to a topic within a namespace. */
    public MsgPublishOut publish(
        String topic,
        final MsgPublishIn msgPublishIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.msgs.publish");
        MsgPublishIn_ body = new MsgPublishIn_(
            msgPublishIn.getNamespace(),
            topic,
            msgPublishIn.getMsgs(),
            msgPublishIn.getIdempotencyKey()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgPublishOut.class
        );
    }
}