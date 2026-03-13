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
import com.svix.coyote.models.RateLimitCheckIn;
import com.svix.coyote.models.RateLimitCheckOut;
import com.svix.coyote.models.RateLimitGetRemainingIn;
import com.svix.coyote.models.RateLimitGetRemainingOut;
import com.svix.coyote.models.RateLimitResetIn;
import com.svix.coyote.models.RateLimitResetOut;

public class RateLimit {
    private final HttpClient client;

    public RateLimit(HttpClient client) {
        this.client = client;
    }

    /** Rate Limiter Check and Consume */
    public RateLimitCheckOut limit(
        final RateLimitCheckIn rateLimitCheckIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/rate-limit/limit");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimitCheckIn,
            RateLimitCheckOut.class
            );
    }

    /** Rate Limiter Get Remaining */
    public RateLimitGetRemainingOut getRemaining(
        final RateLimitGetRemainingIn rateLimitGetRemainingIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/rate-limit/get-remaining");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimitGetRemainingIn,
            RateLimitGetRemainingOut.class
            );
    }

    /** Rate Limiter Reset */
    public RateLimitResetOut reset(
        final RateLimitResetIn rateLimitResetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/rate-limit/reset");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimitResetIn,
            RateLimitResetOut.class
            );
    }
}