// this file is @generated
package com.svix.diom;

import java.util.Map;
import java.util.TreeMap;

import com.svix.diom.apis.Admin;
import com.svix.diom.apis.Cache;
import com.svix.diom.apis.Health;
import com.svix.diom.apis.Idempotency;
import com.svix.diom.apis.Kv;
import com.svix.diom.apis.Msgs;
import com.svix.diom.apis.RateLimit;

import okhttp3.HttpUrl;

public class Diom {
    private final HttpClient httpClient;

    public Diom(String token) {
        this(token, new DiomOptions());
    }

    public Diom(String token, DiomOptions options) {
        if (options.getServerUrl() == null) {
            options.setServerUrl(DiomOptions.DEFAULT_URL);
        }

        HttpUrl parsedUrl = HttpUrl.parse(options.getServerUrl());
        if (parsedUrl == null) {
            throw new IllegalArgumentException("Invalid base url");
        }

        Map<String, String> defaultHeaders = new TreeMap<>();
        defaultHeaders.put("user-agent", "diom-libs/0.1.0/java");
        defaultHeaders.put("Authorization", "Bearer " + token);

        this.httpClient
                = new HttpClient(parsedUrl, defaultHeaders, options.getRetrySchedule());
    }

    public Admin getAdmin() {
        return new Admin(this.httpClient);
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
}