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
import java.util.List;
import java.util.Map;
import java.util.Set;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.coyote.models.AdminAccessPolicyDeleteIn;
import com.svix.coyote.models.AdminAccessPolicyDeleteOut;
import com.svix.coyote.models.AdminAccessPolicyGetIn;
import com.svix.coyote.models.AdminAccessPolicyListIn;
import com.svix.coyote.models.AdminAccessPolicyOut;
import com.svix.coyote.models.AdminAccessPolicyUpsertIn;
import com.svix.coyote.models.AdminAccessPolicyUpsertOut;
import com.svix.coyote.models.ListResponseAdminAccessPolicyOut;

public class AdminAuthPolicy {
    private final HttpClient client;

    public AdminAuthPolicy(HttpClient client) {
        this.client = client;
    }

    /** Create or update an access policy */
    public AdminAccessPolicyUpsertOut upsert(
        final AdminAccessPolicyUpsertIn adminAccessPolicyUpsertIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-policy.upsert");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAccessPolicyUpsertIn,
            AdminAccessPolicyUpsertOut.class
        );
    }

    /** Delete an access policy */
    public AdminAccessPolicyDeleteOut delete(
        final AdminAccessPolicyDeleteIn adminAccessPolicyDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-policy.delete");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAccessPolicyDeleteIn,
            AdminAccessPolicyDeleteOut.class
        );
    }

    /** Get an access policy by ID */
    public AdminAccessPolicyOut get(
        final AdminAccessPolicyGetIn adminAccessPolicyGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-policy.get");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAccessPolicyGetIn,
            AdminAccessPolicyOut.class
        );
    }

    /** List all access policies */
    public ListResponseAdminAccessPolicyOut list(
        final AdminAccessPolicyListIn adminAccessPolicyListIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-policy.list");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminAccessPolicyListIn,
            ListResponseAdminAccessPolicyOut.class
        );
    }

    /** List all access policies */
    public ListResponseAdminAccessPolicyOut list(
        
    ) throws IOException, ApiException {
        return this.list(
            new AdminAccessPolicyListIn()
        );
    }
}