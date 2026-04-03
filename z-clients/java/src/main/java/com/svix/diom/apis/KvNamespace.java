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
import com.svix.diom.models.KvCreateNamespaceIn;
import com.svix.diom.models.KvCreateNamespaceOut;
import com.svix.diom.models.KvGetNamespaceIn;
import com.svix.diom.models.KvGetNamespaceOut;

public class KvNamespace {
    private final HttpClient client;

    public KvNamespace(HttpClient client) {
        this.client = client;
    }

    /** Create KV namespace */
    public KvCreateNamespaceOut create(
        final KvCreateNamespaceIn kvCreateNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.kv.namespace.create");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            kvCreateNamespaceIn,
            KvCreateNamespaceOut.class
        );
    }

    /** Get KV namespace */
    public KvGetNamespaceOut get(
        final KvGetNamespaceIn kvGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.kv.namespace.get");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            kvGetNamespaceIn,
            KvGetNamespaceOut.class
        );
    }
}