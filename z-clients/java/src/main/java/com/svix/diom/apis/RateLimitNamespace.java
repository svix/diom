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
import java.util.List;
import java.util.Map;
import java.util.Set;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.RateLimitConfigureNamespaceIn;
import com.svix.diom.models.RateLimitConfigureNamespaceOut;
import com.svix.diom.models.RateLimitGetNamespaceIn;
import com.svix.diom.models.RateLimitGetNamespaceOut;

public class RateLimitNamespace {
    private final HttpClient client;

    public RateLimitNamespace(HttpClient client) {
        this.client = client;
    }

    /** Configure rate limiter namespace */
    public RateLimitConfigureNamespaceOut configure(
        final RateLimitConfigureNamespaceIn rateLimitConfigureNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.rate-limit.namespace.configure");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimitConfigureNamespaceIn,
            RateLimitConfigureNamespaceOut.class
        );
    }

    /** Get rate limiter namespace */
    public RateLimitGetNamespaceOut get(
        final RateLimitGetNamespaceIn rateLimitGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.rate-limit.namespace.get");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            rateLimitGetNamespaceIn,
            RateLimitGetNamespaceOut.class
        );
    }
}