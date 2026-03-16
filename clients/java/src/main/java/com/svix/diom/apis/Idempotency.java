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
import com.svix.diom.models.IdempotencyCompleteIn;
import com.svix.diom.models.IdempotencyCompleteOut;
import com.svix.diom.models.IdempotencyStartIn;
import com.svix.diom.models.IdempotencyStartOut;

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