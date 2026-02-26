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
import com.svix.coyote.models.KvDeleteIn;
import com.svix.coyote.models.KvDeleteOut;
import com.svix.coyote.models.KvGetIn;
import com.svix.coyote.models.KvGetNamespaceIn;
import com.svix.coyote.models.KvGetNamespaceOut;
import com.svix.coyote.models.KvGetOut;
import com.svix.coyote.models.KvSetIn;
import com.svix.coyote.models.KvSetOut;

public class Kv {
    private final HttpClient client;

    public Kv(HttpClient client) {
        this.client = client;
    }

    /** KV Set */
    public KvSetOut set(
        final KvSetIn kvSetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/kv/set");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            kvSetIn,
            KvSetOut.class
            );
    }

    /** KV Get */
    public KvGetOut get(
        final KvGetIn kvGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/kv/get");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            kvGetIn,
            KvGetOut.class
            );
    }

    /** Get KV namespace */
    public KvGetNamespaceOut getNamespace(
        final KvGetNamespaceIn kvGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/kv/get-namespace");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            kvGetNamespaceIn,
            KvGetNamespaceOut.class
            );
    }

    /** KV Delete */
    public KvDeleteOut delete(
        final KvDeleteIn kvDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/kv/delete");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            kvDeleteIn,
            KvDeleteOut.class
            );
    }
}