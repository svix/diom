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
import com.svix.diom.models.PingOut;

public class Health {
    private final HttpClient client;

    public Health(HttpClient client) {
        this.client = client;
    }

    /** Verify the server is up and running. */
    public PingOut ping(
        
    ) throws DiomException {

        return this.client.executeRequest(
            "GET",
            "/api/v1.health.ping",
            null,
            null,
            PingOut.class
        );
    }

    /** Intentionally return an error */
    public void error(
        
    ) throws DiomException {

        this.client.executeRequest(
            "POST",
            "/api/v1.health.error",
            null,
            null,
            null
        );
    }
}