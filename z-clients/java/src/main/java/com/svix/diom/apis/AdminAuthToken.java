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
import com.svix.diom.models.AdminAuthTokenCreateIn;
import com.svix.diom.models.AdminAuthTokenCreateOut;
import com.svix.diom.models.AdminAuthTokenDeleteIn;
import com.svix.diom.models.AdminAuthTokenDeleteOut;
import com.svix.diom.models.AdminAuthTokenExpireIn;
import com.svix.diom.models.AdminAuthTokenExpireOut;
import com.svix.diom.models.AdminAuthTokenListIn;
import com.svix.diom.models.AdminAuthTokenRotateIn;
import com.svix.diom.models.AdminAuthTokenRotateOut;
import com.svix.diom.models.AdminAuthTokenUpdateIn;
import com.svix.diom.models.AdminAuthTokenUpdateOut;
import com.svix.diom.models.AdminAuthTokenWhoamiIn;
import com.svix.diom.models.AdminAuthTokenWhoamiOut;
import com.svix.diom.models.ListResponseAdminAuthTokenOut;

public class AdminAuthToken {
    private final HttpClient client;

    public AdminAuthToken(HttpClient client) {
        this.client = client;
    }

    /** Create an auth token */
    public AdminAuthTokenCreateOut create(
        final AdminAuthTokenCreateIn adminAuthTokenCreateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-token.create");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAuthTokenCreateIn,
            AdminAuthTokenCreateOut.class
        );
    }

    /** Expire an auth token */
    public AdminAuthTokenExpireOut expire(
        final AdminAuthTokenExpireIn adminAuthTokenExpireIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-token.expire");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAuthTokenExpireIn,
            AdminAuthTokenExpireOut.class
        );
    }

    /** Rotate an auth token, invalidating the old one and issuing a new secret */
    public AdminAuthTokenRotateOut rotate(
        final AdminAuthTokenRotateIn adminAuthTokenRotateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-token.rotate");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAuthTokenRotateIn,
            AdminAuthTokenRotateOut.class
        );
    }

    /** Delete an auth token */
    public AdminAuthTokenDeleteOut delete(
        final AdminAuthTokenDeleteIn adminAuthTokenDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-token.delete");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAuthTokenDeleteIn,
            AdminAuthTokenDeleteOut.class
        );
    }

    /** List auth tokens for a given owner */
    public ListResponseAdminAuthTokenOut list(
        final AdminAuthTokenListIn adminAuthTokenListIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-token.list");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAuthTokenListIn,
            ListResponseAdminAuthTokenOut.class
        );
    }

    /** List auth tokens for a given owner */
    public ListResponseAdminAuthTokenOut list(
        
    ) throws IOException, ApiException {
        return this.list(
            new AdminAuthTokenListIn()
        );
    }

    /** Update an auth token's properties */
    public AdminAuthTokenUpdateOut update(
        final AdminAuthTokenUpdateIn adminAuthTokenUpdateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-token.update");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAuthTokenUpdateIn,
            AdminAuthTokenUpdateOut.class
        );
    }

    /** Return the role of the currently authenticated token */
    public AdminAuthTokenWhoamiOut whoami(
        final AdminAuthTokenWhoamiIn adminAuthTokenWhoamiIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-token.whoami");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAuthTokenWhoamiIn,
            AdminAuthTokenWhoamiOut.class
        );
    }

    /** Return the role of the currently authenticated token */
    public AdminAuthTokenWhoamiOut whoami(
        
    ) throws IOException, ApiException {
        return this.whoami(
            new AdminAuthTokenWhoamiIn()
        );
    }
}