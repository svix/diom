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
import com.svix.diom.models.AdminAccessPolicyConfigureIn;
import com.svix.diom.models.AdminAccessPolicyConfigureOut;
import com.svix.diom.models.AdminAccessPolicyDeleteIn;
import com.svix.diom.models.AdminAccessPolicyDeleteOut;
import com.svix.diom.models.AdminAccessPolicyGetIn;
import com.svix.diom.models.AdminAccessPolicyListIn;
import com.svix.diom.models.AdminAccessPolicyOut;
import com.svix.diom.models.ListResponseAdminAccessPolicyOut;

public class AdminAuthPolicy {
    private final HttpClient client;

    public AdminAuthPolicy(HttpClient client) {
        this.client = client;
    }

    /** Create or update an access policy */
    public AdminAccessPolicyConfigureOut configure(
        final AdminAccessPolicyConfigureIn adminAccessPolicyConfigureIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-policy.configure",
            null,
            adminAccessPolicyConfigureIn,
            AdminAccessPolicyConfigureOut.class
        );
    }

    /** Delete an access policy */
    public AdminAccessPolicyDeleteOut delete(
        final AdminAccessPolicyDeleteIn adminAccessPolicyDeleteIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-policy.delete",
            null,
            adminAccessPolicyDeleteIn,
            AdminAccessPolicyDeleteOut.class
        );
    }

    /** Get an access policy by ID */
    public AdminAccessPolicyOut get(
        final AdminAccessPolicyGetIn adminAccessPolicyGetIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-policy.get",
            null,
            adminAccessPolicyGetIn,
            AdminAccessPolicyOut.class
        );
    }

    /** List all access policies */
    public ListResponseAdminAccessPolicyOut list(
        final AdminAccessPolicyListIn adminAccessPolicyListIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.admin.auth-policy.list",
            null,
            adminAccessPolicyListIn,
            ListResponseAdminAccessPolicyOut.class
        );
    }

    /** List all access policies */
    public ListResponseAdminAccessPolicyOut list(
        
    ) throws DiomException {
        return this.list(
            new AdminAccessPolicyListIn()
        );
    }
}