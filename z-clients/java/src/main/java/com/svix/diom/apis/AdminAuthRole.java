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
import com.svix.diom.models.AdminRoleConfigureIn;
import com.svix.diom.models.AdminRoleConfigureOut;
import com.svix.diom.models.AdminRoleDeleteIn;
import com.svix.diom.models.AdminRoleDeleteOut;
import com.svix.diom.models.AdminRoleGetIn;
import com.svix.diom.models.AdminRoleListIn;
import com.svix.diom.models.AdminRoleOut;
import com.svix.diom.models.ListResponseAdminRoleOut;

public class AdminAuthRole {
    private final HttpClient client;

    public AdminAuthRole(HttpClient client) {
        this.client = client;
    }

    /** Create or update a role */
    public AdminRoleConfigureOut configure(
        final AdminRoleConfigureIn adminRoleConfigureIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-role.configure",
            null,
            adminRoleConfigureIn,
            AdminRoleConfigureOut.class
        );
    }

    /** Delete a role */
    public AdminRoleDeleteOut delete(
        final AdminRoleDeleteIn adminRoleDeleteIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-role.delete",
            null,
            adminRoleDeleteIn,
            AdminRoleDeleteOut.class
        );
    }

    /** Get a role by ID */
    public AdminRoleOut get(
        final AdminRoleGetIn adminRoleGetIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-role.get",
            null,
            adminRoleGetIn,
            AdminRoleOut.class
        );
    }

    /** List all roles */
    public ListResponseAdminRoleOut list(
        final AdminRoleListIn adminRoleListIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-role.list",
            null,
            adminRoleListIn,
            ListResponseAdminRoleOut.class
        );
    }

    /** List all roles */
    public ListResponseAdminRoleOut list(
        
    ) throws DiomException {
        return this.list(
            new AdminRoleListIn()
        );
    }
}