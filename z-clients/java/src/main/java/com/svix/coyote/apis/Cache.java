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
import lombok.Getter;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.coyote.models.CacheDeleteIn;
import com.svix.coyote.models.CacheDeleteOut;
import com.svix.coyote.models.CacheGetIn;
import com.svix.coyote.models.CacheGetOut;
import com.svix.coyote.models.CacheSetIn;
import com.svix.coyote.models.CacheSetOut;
import com.svix.coyote.models.CacheSetIn_;
import com.svix.coyote.models.CacheGetIn_;
import com.svix.coyote.models.CacheDeleteIn_;

public class Cache {
    private final HttpClient client;

    public Cache(HttpClient client) {
        this.client = client;
    }

    /** Cache Set */
    public CacheSetOut set(
        String key,
        final CacheSetIn cacheSetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.cache.set");
        CacheSetIn_ body = new CacheSetIn_(
            cacheSetIn.getNamespace(),
            key,
            cacheSetIn.getValue(),
            cacheSetIn.getTtlMs()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            CacheSetOut.class
        );
    }

    /** Cache Get */
    public CacheGetOut get(
        String key,
        final CacheGetIn cacheGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.cache.get");
        CacheGetIn_ body = new CacheGetIn_(
            cacheGetIn.getNamespace(),
            key,
            cacheGetIn.getConsistency()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            CacheGetOut.class
        );
    }

    /** Cache Get */
    public CacheGetOut get(
        String key
    ) throws IOException, ApiException {
        return this.get(
            key,
            new CacheGetIn()
        );
    }

    /** Cache Delete */
    public CacheDeleteOut delete(
        String key,
        final CacheDeleteIn cacheDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.cache.delete");
        CacheDeleteIn_ body = new CacheDeleteIn_(
            cacheDeleteIn.getNamespace(),
            key
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            CacheDeleteOut.class
        );
    }

    /** Cache Delete */
    public CacheDeleteOut delete(
        String key
    ) throws IOException, ApiException {
        return this.delete(
            key,
            new CacheDeleteIn()
        );
    }
}