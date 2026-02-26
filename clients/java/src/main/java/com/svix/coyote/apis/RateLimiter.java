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
import com.svix.coyote.models.RateLimiterCheckIn;
import com.svix.coyote.models.RateLimiterCheckOut;
import com.svix.coyote.models.RateLimiterGetRemainingIn;
import com.svix.coyote.models.RateLimiterGetRemainingOut;

public class RateLimiter {
    private final HttpClient client;

    public RateLimiter(HttpClient client) {
        this.client = client;
    }

    /** Rate Limiter Check and Consume */
    public RateLimiterCheckOut limit(
        final RateLimiterCheckIn rateLimiterCheckIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/rate-limiter/limit");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimiterCheckIn,
            RateLimiterCheckOut.class
            );
    }

    /** Rate Limiter Get Remaining */
    public RateLimiterGetRemainingOut getRemaining(
        final RateLimiterGetRemainingIn rateLimiterGetRemainingIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/rate-limiter/get-remaining");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimiterGetRemainingIn,
            RateLimiterGetRemainingOut.class
            );
    }
}