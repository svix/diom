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
import com.svix.diom.models.CacheConfigureNamespaceIn;
import com.svix.diom.models.CacheConfigureNamespaceOut;
import com.svix.diom.models.CacheGetNamespaceIn;
import com.svix.diom.models.CacheGetNamespaceOut;

public class CacheNamespace {
    private final HttpClient client;

    public CacheNamespace(HttpClient client) {
        this.client = client;
    }

    /** Configure cache namespace */
    public CacheConfigureNamespaceOut configure(
        final CacheConfigureNamespaceIn cacheConfigureNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.cache.namespace.configure");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheConfigureNamespaceIn,
            CacheConfigureNamespaceOut.class
        );
    }

    /** Get cache namespace */
    public CacheGetNamespaceOut get(
        final CacheGetNamespaceIn cacheGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.cache.namespace.get");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            cacheGetNamespaceIn,
            CacheGetNamespaceOut.class
        );
    }
}