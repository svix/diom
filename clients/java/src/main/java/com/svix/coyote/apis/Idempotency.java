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
import com.svix.coyote.models.IdempotencyAbortIn;
import com.svix.coyote.models.IdempotencyAbortOut;
import com.svix.coyote.models.IdempotencyGetNamespaceIn;
import com.svix.coyote.models.IdempotencyGetNamespaceOut;

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

    /** Get idempotency namespace */
    public IdempotencyGetNamespaceOut getNamespace(
        final IdempotencyGetNamespaceIn idempotencyGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/idempotency/get-namespace");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            idempotencyGetNamespaceIn,
            IdempotencyGetNamespaceOut.class
            );
    }
}