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
import com.svix.coyote.models.CacheCreateNamespaceIn;
import com.svix.coyote.models.CacheCreateNamespaceOut;
import com.svix.coyote.models.CacheGetNamespaceIn;
import com.svix.coyote.models.CacheGetNamespaceOut;

public class CacheNamespace {
    private final HttpClient client;

    public CacheNamespace(HttpClient client) {
        this.client = client;
    }

    /** Create cache namespace */
    public CacheCreateNamespaceOut create(
        final CacheCreateNamespaceIn cacheCreateNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/cache/namespace/create");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheCreateNamespaceIn,
            CacheCreateNamespaceOut.class
        );
    }

    /** Get cache namespace */
    public CacheGetNamespaceOut get(
        final CacheGetNamespaceIn cacheGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/cache/namespace/get");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheGetNamespaceIn,
            CacheGetNamespaceOut.class
        );
    }
}