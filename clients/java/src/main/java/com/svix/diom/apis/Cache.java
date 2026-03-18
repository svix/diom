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
import lombok.Getter;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.CacheDeleteIn;
import com.svix.diom.models.CacheDeleteOut;
import com.svix.diom.models.CacheGetIn;
import com.svix.diom.models.CacheGetOut;
import com.svix.diom.models.CacheSetIn;
import com.svix.diom.models.CacheSetOut;

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