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
import com.svix.coyote.models.RateLimiterCreateNamespaceIn;
import com.svix.coyote.models.RateLimiterCreateNamespaceOut;
import com.svix.coyote.models.RateLimiterGetNamespaceIn;
import com.svix.coyote.models.RateLimiterGetNamespaceOut;

public class RateLimiterNamespace {
    private final HttpClient client;

    public RateLimiterNamespace(HttpClient client) {
        this.client = client;
    }

    /** Create rate limiter namespace */
    public RateLimiterCreateNamespaceOut create(
        final RateLimiterCreateNamespaceIn rateLimiterCreateNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/rate-limit/namespace/create");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimiterCreateNamespaceIn,
            RateLimiterCreateNamespaceOut.class
            );
    }

    /** Get rate limiter namespace */
    public RateLimiterGetNamespaceOut get(
        final RateLimiterGetNamespaceIn rateLimiterGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/rate-limit/namespace/get");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimiterGetNamespaceIn,
            RateLimiterGetNamespaceOut.class
            );
    }
}