// this file is @generated
package com.svix.diom;

import java.util.Map;
import java.util.TreeMap;

import com.svix.diom.apis.Admin;
import com.svix.diom.apis.Cache;
import com.svix.diom.apis.ClusterAdmin;
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
        defaultHeaders.put("user-agent", "diom-libs/" + Version.VERSION + "/java");
        defaultHeaders.put("Authorization", "Bearer " + token);

        this.httpClient
                = new HttpClient(parsedUrl, defaultHeaders, options.getRetrySchedule());
    }

    public Admin admin() {
        return new Admin(this.httpClient);
    }

    public Cache cache() {
        return new Cache(this.httpClient);
    }

    public ClusterAdmin clusterAdmin() {
        return new ClusterAdmin(this.httpClient);
    }

    public Health health() {
        return new Health(this.httpClient);
    }

    public Idempotency idempotency() {
        return new Idempotency(this.httpClient);
    }

    public Kv kv() {
        return new Kv(this.httpClient);
    }

    public Msgs msgs() {
        return new Msgs(this.httpClient);
    }

    public RateLimit rateLimit() {
        return new RateLimit(this.httpClient);
    }
}