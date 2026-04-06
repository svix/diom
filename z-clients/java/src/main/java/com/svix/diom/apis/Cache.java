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
import lombok.Getter;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.CacheDeleteIn;
import com.svix.diom.models.CacheDeleteOut;
import com.svix.diom.models.CacheGetIn;
import com.svix.diom.models.CacheGetOut;
import com.svix.diom.models.CacheSetIn;
import com.svix.diom.models.CacheSetOut;
import com.svix.diom.models.CacheSetIn_;
import com.svix.diom.models.CacheGetIn_;
import com.svix.diom.models.CacheDeleteIn_;

public class Cache {
    private final HttpClient client;

    public Cache(HttpClient client) {
        this.client = client;
    }

    /** Cache Set */
    public CacheSetOut set(
        String key,
        List<Byte> value,
        final CacheSetIn cacheSetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.cache.set");
        CacheSetIn_ body = new CacheSetIn_(
            cacheSetIn.getNamespace(),
            key,
            value,
            cacheSetIn.getTtl()
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