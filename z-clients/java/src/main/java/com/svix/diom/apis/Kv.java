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
import com.svix.diom.models.KvDeleteIn;
import com.svix.diom.models.KvDeleteOut;
import com.svix.diom.models.KvGetIn;
import com.svix.diom.models.KvGetOut;
import com.svix.diom.models.KvSetIn;
import com.svix.diom.models.KvSetOut;
import com.svix.diom.models.KvSetIn_;
import com.svix.diom.models.KvGetIn_;
import com.svix.diom.models.KvDeleteIn_;

public class Kv {
    private final HttpClient client;

    public Kv(HttpClient client) {
        this.client = client;
    }

    public KvNamespace namespace() {
        return new KvNamespace(this.client);
    }

    /** KV Set */
    public KvSetOut set(
        String key,
        byte[] value,
        final KvSetIn kvSetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.kv.set");
        KvSetIn_ body = new KvSetIn_(
            kvSetIn.getNamespace(),
            key,
            value,
            kvSetIn.getTtl(),
            kvSetIn.getBehavior(),
            kvSetIn.getVersion()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            KvSetOut.class
        );
    }

    /** KV Set */
    public KvSetOut set(
        String key,
        byte[] value
    ) throws IOException, ApiException {
        return this.set(
            key,
            value,
            new KvSetIn()
        );
    }

    /** KV Get */
    public KvGetOut get(
        String key,
        final KvGetIn kvGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.kv.get");
        KvGetIn_ body = new KvGetIn_(
            kvGetIn.getNamespace(),
            key,
            kvGetIn.getConsistency()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            KvGetOut.class
        );
    }

    /** KV Get */
    public KvGetOut get(
        String key
    ) throws IOException, ApiException {
        return this.get(
            key,
            new KvGetIn()
        );
    }

    /** KV Delete */
    public KvDeleteOut delete(
        String key,
        final KvDeleteIn kvDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.kv.delete");
        KvDeleteIn_ body = new KvDeleteIn_(
            kvDeleteIn.getNamespace(),
            key,
            kvDeleteIn.getVersion()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            KvDeleteOut.class
        );
    }

    /** KV Delete */
    public KvDeleteOut delete(
        String key
    ) throws IOException, ApiException {
        return this.delete(
            key,
            new KvDeleteIn()
        );
    }
}