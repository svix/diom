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
import com.svix.diom.models.KvDeleteIn;
import com.svix.diom.models.KvDeleteOut;
import com.svix.diom.models.KvGetIn;
import com.svix.diom.models.KvGetOut;
import com.svix.diom.models.KvSetIn;
import com.svix.diom.models.KvSetOut;

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