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
import com.svix.diom.models.CreateMsgTopicIn;
import com.svix.diom.models.CreateMsgTopicOut;
import com.svix.diom.models.GetMsgTopicIn;
import com.svix.diom.models.GetMsgTopicOut;

public class MsgsTopic {
    private final HttpClient client;

    public MsgsTopic(HttpClient client) {
        this.client = client;
    }

    /** Upserts a new message topic with the given name. */
    public CreateMsgTopicOut create(
        final CreateMsgTopicIn createMsgTopicIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/topic/create");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            createMsgTopicIn,
            CreateMsgTopicOut.class
            );
    }

    /** Get message topic with given name. */
    public GetMsgTopicOut get(
        final GetMsgTopicIn getMsgTopicIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/topic/get");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            getMsgTopicIn,
            GetMsgTopicOut.class
            );
    }
}