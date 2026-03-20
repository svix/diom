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
import com.svix.diom.models.AuthTokenCreateIn;
import com.svix.diom.models.AuthTokenCreateOut;
import com.svix.diom.models.AuthTokenDeleteIn;
import com.svix.diom.models.AuthTokenDeleteOut;
import com.svix.diom.models.AuthTokenExpireIn;
import com.svix.diom.models.AuthTokenExpireOut;
import com.svix.diom.models.AuthTokenListIn;
import com.svix.diom.models.AuthTokenUpdateIn;
import com.svix.diom.models.AuthTokenUpdateOut;
import com.svix.diom.models.AuthTokenVerifyIn;
import com.svix.diom.models.AuthTokenVerifyOut;
import com.svix.diom.models.ListResponseAuthTokenOut;

public class AuthToken {
    private final HttpClient client;

    public AuthToken(HttpClient client) {
        this.client = client;
    }

    /** Create Auth Token */
    public AuthTokenCreateOut create(
        final AuthTokenCreateIn authTokenCreateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth-token/create");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenCreateIn,
            AuthTokenCreateOut.class
        );
    }

    /** Expire Auth Token */
    public AuthTokenExpireOut expire(
        final AuthTokenExpireIn authTokenExpireIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth-token/expire");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenExpireIn,
            AuthTokenExpireOut.class
        );
    }

    /** Delete Auth Token */
    public AuthTokenDeleteOut delete(
        final AuthTokenDeleteIn authTokenDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth-token/delete");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenDeleteIn,
            AuthTokenDeleteOut.class
        );
    }

    /** Verify Auth Token */
    public AuthTokenVerifyOut verify(
        final AuthTokenVerifyIn authTokenVerifyIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth-token/verify");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenVerifyIn,
            AuthTokenVerifyOut.class
        );
    }

    /** List Auth Tokens */
    public ListResponseAuthTokenOut list(
        final AuthTokenListIn authTokenListIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth-token/list");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenListIn,
            ListResponseAuthTokenOut.class
        );
    }

    /** Update Auth Token */
    public AuthTokenUpdateOut update(
        final AuthTokenUpdateIn authTokenUpdateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth-token/update");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenUpdateIn,
            AuthTokenUpdateOut.class
        );
    }
}