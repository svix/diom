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
import com.svix.coyote.models.CreateMsgTopicIn;
import com.svix.coyote.models.CreateMsgTopicOut;
import com.svix.coyote.models.GetMsgTopicIn;
import com.svix.coyote.models.GetMsgTopicOut;

public class Msgs {
    private final HttpClient client;

    public Msgs(HttpClient client) {
        this.client = client;
    }
}