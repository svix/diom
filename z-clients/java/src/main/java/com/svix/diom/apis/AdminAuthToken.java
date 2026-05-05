// this file is @generated
package com.svix.diom.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.diom.DiomException;
import com.svix.diom.HttpClient;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
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
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-token.create",
            null,
            adminAuthTokenCreateIn,
            AdminAuthTokenCreateOut.class
        );
    }

    /** Expire an auth token */
    public AdminAuthTokenExpireOut expire(
        final AdminAuthTokenExpireIn adminAuthTokenExpireIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-token.expire",
            null,
            adminAuthTokenExpireIn,
            AdminAuthTokenExpireOut.class
        );
    }

    /** Rotate an auth token, invalidating the old one and issuing a new secret */
    public AdminAuthTokenRotateOut rotate(
        final AdminAuthTokenRotateIn adminAuthTokenRotateIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-token.rotate",
            null,
            adminAuthTokenRotateIn,
            AdminAuthTokenRotateOut.class
        );
    }

    /** Delete an auth token */
    public AdminAuthTokenDeleteOut delete(
        final AdminAuthTokenDeleteIn adminAuthTokenDeleteIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-token.delete",
            null,
            adminAuthTokenDeleteIn,
            AdminAuthTokenDeleteOut.class
        );
    }

    /** List auth tokens for a given owner */
    public ListResponseAdminAuthTokenOut list(
        final AdminAuthTokenListIn adminAuthTokenListIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-token.list",
            null,
            adminAuthTokenListIn,
            ListResponseAdminAuthTokenOut.class
        );
    }

    /** List auth tokens for a given owner */
    public ListResponseAdminAuthTokenOut list(
        
    ) throws DiomException {
        return this.list(
            new AdminAuthTokenListIn()
        );
    }

    /** Update an auth token's properties */
    public AdminAuthTokenUpdateOut update(
        final AdminAuthTokenUpdateIn adminAuthTokenUpdateIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-token.update",
            null,
            adminAuthTokenUpdateIn,
            AdminAuthTokenUpdateOut.class
        );
    }

    /** Return the role of the currently authenticated token */
    public AdminAuthTokenWhoamiOut whoami(
        final AdminAuthTokenWhoamiIn adminAuthTokenWhoamiIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-token.whoami",
            null,
            adminAuthTokenWhoamiIn,
            AdminAuthTokenWhoamiOut.class
        );
    }

    /** Return the role of the currently authenticated token */
    public AdminAuthTokenWhoamiOut whoami(
        
    ) throws DiomException {
        return this.whoami(
            new AdminAuthTokenWhoamiIn()
        );
    }
}