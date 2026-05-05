// this file is @generated
package com.svix.diom.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.diom.DiomException;
import com.svix.diom.HttpClient;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
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

    public IdempotencyNamespace namespace() {
        return new IdempotencyNamespace(this.client);
    }

    /** Start an idempotent request */
    public IdempotencyStartOut start(
        String key,
        final IdempotencyStartIn idempotencyStartIn
    ) throws DiomException {
        IdempotencyStartIn_ body = new IdempotencyStartIn_(
            idempotencyStartIn.getNamespace(),
            key,
            idempotencyStartIn.getLockPeriod()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.idempotency.start",
            null,
            body,
            IdempotencyStartOut.class
        );
    }

    /** Complete an idempotent request with a response */
    public IdempotencyCompleteOut complete(
        String key,
        final IdempotencyCompleteIn idempotencyCompleteIn
    ) throws DiomException {
        IdempotencyCompleteIn_ body = new IdempotencyCompleteIn_(
            idempotencyCompleteIn.getNamespace(),
            key,
            idempotencyCompleteIn.getResponse(),
            idempotencyCompleteIn.getContext(),
            idempotencyCompleteIn.getTtl()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.idempotency.complete",
            null,
            body,
            IdempotencyCompleteOut.class
        );
    }

    /** Abandon an idempotent request (remove lock without saving response) */
    public IdempotencyAbortOut abort(
        String key,
        final IdempotencyAbortIn idempotencyAbortIn
    ) throws DiomException {
        IdempotencyAbortIn_ body = new IdempotencyAbortIn_(
            idempotencyAbortIn.getNamespace(),
            key
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.idempotency.abort",
            null,
            body,
            IdempotencyAbortOut.class
        );
    }

    /** Abandon an idempotent request (remove lock without saving response) */
    public IdempotencyAbortOut abort(
        String key
    ) throws DiomException {
        return this.abort(
            key,
            new IdempotencyAbortIn()
        );
    }
}