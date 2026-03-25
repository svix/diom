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
import com.svix.diom.models.IdempotencyStartIn_;
import com.svix.diom.models.IdempotencyCompleteIn_;
import com.svix.diom.models.IdempotencyAbortIn_;

public class Idempotency {
    private final HttpClient client;

    public Idempotency(HttpClient client) {
        this.client = client;
    }

    /** Start an idempotent request */
    public IdempotencyStartOut start(
        String key,
        final IdempotencyStartIn idempotencyStartIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.idempotency.start");
        IdempotencyStartIn_ body = new IdempotencyStartIn_(
            idempotencyStartIn.getNamespace(),
            key,
            idempotencyStartIn.getTtl()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            IdempotencyStartOut.class
        );
    }

    /** Complete an idempotent request with a response */
    public IdempotencyCompleteOut complete(
        String key,
        final IdempotencyCompleteIn idempotencyCompleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.idempotency.complete");
        IdempotencyCompleteIn_ body = new IdempotencyCompleteIn_(
            idempotencyCompleteIn.getNamespace(),
            key,
            idempotencyCompleteIn.getResponse(),
            idempotencyCompleteIn.getTtl()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            IdempotencyCompleteOut.class
        );
    }

    /** Abandon an idempotent request (remove lock without saving response) */
    public IdempotencyAbortOut abort(
        String key,
        final IdempotencyAbortIn idempotencyAbortIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.idempotency.abort");
        IdempotencyAbortIn_ body = new IdempotencyAbortIn_(
            idempotencyAbortIn.getNamespace(),
            key
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            IdempotencyAbortOut.class
        );
    }

    /** Abandon an idempotent request (remove lock without saving response) */
    public IdempotencyAbortOut abort(
        String key
    ) throws IOException, ApiException {
        return this.abort(
            key,
            new IdempotencyAbortIn()
        );
    }
}