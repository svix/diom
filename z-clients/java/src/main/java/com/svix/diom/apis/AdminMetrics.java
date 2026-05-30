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
import com.svix.diom.models.GetMetricsOut;

public class AdminMetrics {
    private final HttpClient client;

    public AdminMetrics(HttpClient client) {
        this.client = client;
    }

    /** Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver) */
    public GetMetricsOut get(
        
    ) throws DiomException {

        return this.client.executeRequest(
            "GET",
            "/api/v1.admin.metrics.get",
            null,
            null,
            GetMetricsOut.class
        );
    }
}