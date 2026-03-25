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
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.AuthTokenCreateNamespaceIn;
import com.svix.diom.models.AuthTokenCreateNamespaceOut;
import com.svix.diom.models.AuthTokenGetNamespaceIn;
import com.svix.diom.models.AuthTokenGetNamespaceOut;

public class AuthTokenNamespace {
    private final HttpClient client;

    public AuthTokenNamespace(HttpClient client) {
        this.client = client;
    }

    /** Create Auth Token namespace */
    public AuthTokenCreateNamespaceOut create(
        final AuthTokenCreateNamespaceIn authTokenCreateNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.auth-token.namespace.create");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenCreateNamespaceIn,
            AuthTokenCreateNamespaceOut.class
        );
    }

    /** Get Auth Token namespace */
    public AuthTokenGetNamespaceOut get(
        final AuthTokenGetNamespaceIn authTokenGetNamespaceIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.auth-token.namespace.get");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenGetNamespaceIn,
            AuthTokenGetNamespaceOut.class
        );
    }
}