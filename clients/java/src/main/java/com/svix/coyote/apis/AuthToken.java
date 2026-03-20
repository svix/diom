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
import com.svix.coyote.models.AuthTokenCreateIn;
import com.svix.coyote.models.AuthTokenCreateOut;
import com.svix.coyote.models.AuthTokenDeleteIn;
import com.svix.coyote.models.AuthTokenDeleteOut;
import com.svix.coyote.models.AuthTokenExpireIn;
import com.svix.coyote.models.AuthTokenExpireOut;
import com.svix.coyote.models.AuthTokenListIn;
import com.svix.coyote.models.AuthTokenRotateIn;
import com.svix.coyote.models.AuthTokenRotateOut;
import com.svix.coyote.models.AuthTokenUpdateIn;
import com.svix.coyote.models.AuthTokenUpdateOut;
import com.svix.coyote.models.AuthTokenVerifyIn;
import com.svix.coyote.models.AuthTokenVerifyOut;
import com.svix.coyote.models.ListResponseAuthTokenOut;

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

    /** Rotate Auth Token */
    public AuthTokenRotateOut rotate(
        final AuthTokenRotateIn authTokenRotateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth-token/rotate");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            authTokenRotateIn,
            AuthTokenRotateOut.class
        );
    }
}