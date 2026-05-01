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

public class Admin {
    private final HttpClient client;

    public Admin(HttpClient client) {
        this.client = client;
    }

    public AdminAuthPolicy authPolicy() {
        return new AdminAuthPolicy(this.client);
    }

    public AdminAuthRole authRole() {
        return new AdminAuthRole(this.client);
    }

    public AdminAuthToken authToken() {
        return new AdminAuthToken(this.client);
    }

    public AdminMetrics metrics() {
        return new AdminMetrics(this.client);
    }
}