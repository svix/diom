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
import com.svix.diom.models.RateLimitCheckIn;
import com.svix.diom.models.RateLimitCheckOut;
import com.svix.diom.models.RateLimitGetRemainingIn;
import com.svix.diom.models.RateLimitGetRemainingOut;
import com.svix.diom.models.RateLimitResetIn;
import com.svix.diom.models.RateLimitResetOut;

public class RateLimit {
    private final HttpClient client;

    public RateLimit(HttpClient client) {
        this.client = client;
    }

    public RateLimitNamespace namespace() {
        return new RateLimitNamespace(this.client);
    }

    /** Rate Limiter Check and Consume */
    public RateLimitCheckOut limit(
        final RateLimitCheckIn rateLimitCheckIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.rate-limit.limit",
            null,
            rateLimitCheckIn,
            RateLimitCheckOut.class
        );
    }

    /** Rate Limiter Get Remaining */
    public RateLimitGetRemainingOut getRemaining(
        final RateLimitGetRemainingIn rateLimitGetRemainingIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.rate-limit.get-remaining",
            null,
            rateLimitGetRemainingIn,
            RateLimitGetRemainingOut.class
        );
    }

    /** Rate Limiter Reset */
    public RateLimitResetOut reset(
        final RateLimitResetIn rateLimitResetIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.rate-limit.reset",
            null,
            rateLimitResetIn,
            RateLimitResetOut.class
        );
    }
}