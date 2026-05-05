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
import com.svix.diom.models.KvConfigureNamespaceIn;
import com.svix.diom.models.KvConfigureNamespaceOut;
import com.svix.diom.models.KvGetNamespaceIn;
import com.svix.diom.models.KvGetNamespaceOut;

public class KvNamespace {
    private final HttpClient client;

    public KvNamespace(HttpClient client) {
        this.client = client;
    }

    /** Configure KV namespace */
    public KvConfigureNamespaceOut configure(
        final KvConfigureNamespaceIn kvConfigureNamespaceIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.kv.namespace.configure",
            null,
            kvConfigureNamespaceIn,
            KvConfigureNamespaceOut.class
        );
    }

    /** Get KV namespace */
    public KvGetNamespaceOut get(
        final KvGetNamespaceIn kvGetNamespaceIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.kv.namespace.get",
            null,
            kvGetNamespaceIn,
            KvGetNamespaceOut.class
        );
    }
}