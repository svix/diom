// this file is @generated
package com.svix.coyote;

import java.util.Map;
import java.util.TreeMap;

import com.svix.coyote.apis.Admin;
import com.svix.coyote.apis.AuthToken;
import com.svix.coyote.apis.Cache;
import com.svix.coyote.apis.Health;
import com.svix.coyote.apis.Idempotency;
import com.svix.coyote.apis.Kv;
import com.svix.coyote.apis.Msgs;
import com.svix.coyote.apis.RateLimit;
import com.svix.coyote.apis.Transformations;

import okhttp3.HttpUrl;

public class Coyote {
    private final HttpClient httpClient;

    public Coyote(String token) {
        this(token, new CoyoteOptions());
    }

    public Coyote(String token, CoyoteOptions options) {
        if (options.getServerUrl() == null) {
            options.setServerUrl(CoyoteOptions.DEFAULT_URL);
        }

        HttpUrl parsedUrl = HttpUrl.parse(options.getServerUrl());
        if (parsedUrl == null) {
            throw new IllegalArgumentException("Invalid base url");
        }

        Map<String, String> defaultHeaders = new TreeMap<>();
        defaultHeaders.put("user-agent", "coyote-libs/0.1.0/java");
        defaultHeaders.put("Authorization", "Bearer " + token);

        this.httpClient
                = new HttpClient(parsedUrl, defaultHeaders, options.getRetrySchedule());
    }

    public Admin getAdmin() {
        return new Admin(this.httpClient);
    }

    public AuthToken getAuthToken() {
        return new AuthToken(this.httpClient);
    }

    public Cache getCache() {
        return new Cache(this.httpClient);
    }

    public Health getHealth() {
        return new Health(this.httpClient);
    }

    public Idempotency getIdempotency() {
        return new Idempotency(this.httpClient);
    }

    public Kv getKv() {
        return new Kv(this.httpClient);
    }

    public Msgs getMsgs() {
        return new Msgs(this.httpClient);
    }

    public RateLimit getRateLimit() {
        return new RateLimit(this.httpClient);
    }

    public Transformations getTransformations() {
        return new Transformations(this.httpClient);
    }
}