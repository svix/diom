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

public class Admin {
    private final HttpClient client;

    public Admin(HttpClient client) {
        this.client = client;
    }

    public AdminAuthPolicy getAuthPolicy() {
        return new AdminAuthPolicy(this.client);
    }

    public AdminAuthRole getAuthRole() {
        return new AdminAuthRole(this.client);
    }

    public AdminAuthToken getAuthToken() {
        return new AdminAuthToken(this.client);
    }

    public AdminCluster getCluster() {
        return new AdminCluster(this.client);
    }
}