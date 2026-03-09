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
import com.svix.diom.models.IdempotencyAbortIn;
import com.svix.diom.models.IdempotencyAbortOut;

public class Idempotency {
    private final HttpClient client;

    public Idempotency(HttpClient client) {
        this.client = client;
    }

    /** Abandon an idempotent request (remove lock without saving response) */
    public IdempotencyAbortOut abort(
        final IdempotencyAbortIn idempotencyAbortIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/idempotency/abort");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            idempotencyAbortIn,
            IdempotencyAbortOut.class
            );
    }
}