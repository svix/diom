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
import com.svix.coyote.models.CacheDeleteIn;
import com.svix.coyote.models.CacheDeleteOut;
import com.svix.coyote.models.CacheGetIn;
import com.svix.coyote.models.CacheGetNamespaceIn;
import com.svix.coyote.models.CacheGetNamespaceOut;
import com.svix.coyote.models.CacheGetOut;
import com.svix.coyote.models.CacheSetIn;
import com.svix.coyote.models.CacheSetOut;

public class Cache {
    private final HttpClient client;

    public Cache(HttpClient client) {
        this.client = client;
    }

    /** Cache Set */
    public CacheSetOut set(
        final CacheSetIn cacheSetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/cache/set");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheSetIn,
            CacheSetOut.class
            );
    }

    /** Cache Get */
    public CacheGetOut get(
        final CacheGetIn cacheGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/cache/get");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheGetIn,
            CacheGetOut.class
            );
    }

    /** Get cache namespace */
    public CacheGetNamespaceOut getNamespace(
        final CacheGetNamespaceIn cacheGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/cache/get-namespace");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheGetNamespaceIn,
            CacheGetNamespaceOut.class
            );
    }

    /** Cache Delete */
    public CacheDeleteOut delete(
        final CacheDeleteIn cacheDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/cache/delete");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheDeleteIn,
            CacheDeleteOut.class
            );
    }
}