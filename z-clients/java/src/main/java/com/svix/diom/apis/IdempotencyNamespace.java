// this file is @generated
package com.svix.diom.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.diom.ApiException;
import com.svix.diom.HttpClient;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import com.svix.diom.models.IdempotencyConfigureNamespaceIn;
import com.svix.diom.models.IdempotencyConfigureNamespaceOut;
import com.svix.diom.models.IdempotencyGetNamespaceIn;
import com.svix.diom.models.IdempotencyGetNamespaceOut;

public class IdempotencyNamespace {
    private final HttpClient client;

    public IdempotencyNamespace(HttpClient client) {
        this.client = client;
    }

    /** Configure idempotency namespace */
    public IdempotencyConfigureNamespaceOut configure(
        final IdempotencyConfigureNamespaceIn idempotencyConfigureNamespaceIn
    ) throws IOException, ApiException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.idempotency.namespace.configure",
            null,
            idempotencyConfigureNamespaceIn,
            IdempotencyConfigureNamespaceOut.class
        );
    }

    /** Get idempotency namespace */
    public IdempotencyGetNamespaceOut get(
        final IdempotencyGetNamespaceIn idempotencyGetNamespaceIn
    ) throws IOException, ApiException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.idempotency.namespace.get",
            null,
            idempotencyGetNamespaceIn,
            IdempotencyGetNamespaceOut.class
        );
    }
}