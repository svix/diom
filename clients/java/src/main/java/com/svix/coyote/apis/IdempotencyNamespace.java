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
import com.svix.coyote.models.IdempotencyCreateNamespaceIn;
import com.svix.coyote.models.IdempotencyCreateNamespaceOut;
import com.svix.coyote.models.IdempotencyGetNamespaceIn;
import com.svix.coyote.models.IdempotencyGetNamespaceOut;

public class IdempotencyNamespace {
    private final HttpClient client;

    public IdempotencyNamespace(HttpClient client) {
        this.client = client;
    }

    /** Create idempotency namespace */
    public IdempotencyCreateNamespaceOut create(
        final IdempotencyCreateNamespaceIn idempotencyCreateNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/idempotency/namespace/create");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            idempotencyCreateNamespaceIn,
            IdempotencyCreateNamespaceOut.class
        );
    }

    /** Get idempotency namespace */
    public IdempotencyGetNamespaceOut get(
        final IdempotencyGetNamespaceIn idempotencyGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/idempotency/namespace/get");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            idempotencyGetNamespaceIn,
            IdempotencyGetNamespaceOut.class
        );
    }
}