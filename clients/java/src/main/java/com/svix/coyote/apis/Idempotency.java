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
import com.svix.coyote.models.IdempotencyAbortIn;
import com.svix.coyote.models.IdempotencyAbortOut;
import com.svix.coyote.models.IdempotencyCompleteIn;
import com.svix.coyote.models.IdempotencyCompleteOut;
import com.svix.coyote.models.IdempotencyStartIn;
import com.svix.coyote.models.IdempotencyStartOut;

public class Idempotency {
    private final HttpClient client;

    public Idempotency(HttpClient client) {
        this.client = client;
    }

    /** Start an idempotent request */
    public IdempotencyStartOut start(
        final IdempotencyStartIn idempotencyStartIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/idempotency/start");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            idempotencyStartIn,
            IdempotencyStartOut.class
            );
    }

    /** Complete an idempotent request with a response */
    public IdempotencyCompleteOut complete(
        final IdempotencyCompleteIn idempotencyCompleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/idempotency/complete");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            idempotencyCompleteIn,
            IdempotencyCompleteOut.class
            );
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