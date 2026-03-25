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
import com.svix.diom.models.TransformIn;
import com.svix.diom.models.TransformOut;

public class Transformations {
    private final HttpClient client;

    public Transformations(HttpClient client) {
        this.client = client;
    }

    /** Execute a JavaScript transformation script against a payload and return the result. */
    public TransformOut execute(
        final TransformIn transformIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.transformations.execute");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            transformIn,
            TransformOut.class
        );
    }
}