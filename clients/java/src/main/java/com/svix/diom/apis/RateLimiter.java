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
import com.svix.diom.models.RateLimiterCheckIn;
import com.svix.diom.models.RateLimiterCheckOut;
import com.svix.diom.models.RateLimiterGetRemainingIn;
import com.svix.diom.models.RateLimiterGetRemainingOut;

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