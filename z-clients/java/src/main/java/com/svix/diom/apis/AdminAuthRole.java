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
import com.svix.diom.models.AdminRoleDeleteIn;
import com.svix.diom.models.AdminRoleDeleteOut;
import com.svix.diom.models.AdminRoleGetIn;
import com.svix.diom.models.AdminRoleListIn;
import com.svix.diom.models.AdminRoleOut;
import com.svix.diom.models.AdminRoleUpsertIn;
import com.svix.diom.models.AdminRoleUpsertOut;
import com.svix.diom.models.ListResponseAdminRoleOut;

public class AdminAuthRole {
    private final HttpClient client;

    public AdminAuthRole(HttpClient client) {
        this.client = client;
    }

    /** Create or update a role */
    public AdminRoleUpsertOut upsert(
        final AdminRoleUpsertIn adminRoleUpsertIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-role.upsert");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminRoleUpsertIn,
            AdminRoleUpsertOut.class
        );
    }

    /** Delete a role */
    public AdminRoleDeleteOut delete(
        final AdminRoleDeleteIn adminRoleDeleteIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-role.delete");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminRoleDeleteIn,
            AdminRoleDeleteOut.class
        );
    }

    /** Get a role by ID */
    public AdminRoleOut get(
        final AdminRoleGetIn adminRoleGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-role.get");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminRoleGetIn,
            AdminRoleOut.class
        );
    }

    /** List all roles */
    public ListResponseAdminRoleOut list(
        final AdminRoleListIn adminRoleListIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.admin.auth-role.list");

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            adminRoleListIn,
            ListResponseAdminRoleOut.class
        );
    }

    /** List all roles */
    public ListResponseAdminRoleOut list(
        
    ) throws IOException, ApiException {
        return this.list(
            new AdminRoleListIn()
        );
    }
}