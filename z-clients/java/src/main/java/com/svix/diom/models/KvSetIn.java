// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.annotation.JsonFilter;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.diom.Utils;
import java.util.Map;
import java.util.Set;
import java.util.List;
import java.util.Optional;
import java.util.HashMap;
import java.time.OffsetDateTime;
import java.util.LinkedHashSet;
import java.util.ArrayList;
import java.net.URI;
import java.util.Objects;
import lombok.EqualsAndHashCode;
import lombok.ToString;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class KvSetIn {
    @JsonProperty private String namespace;
    @JsonProperty("ttl_ms") private Long ttlMs;
    @JsonProperty private OperationBehavior behavior;
    @JsonProperty private Long version;
    public KvSetIn() {}

    public KvSetIn namespace(String namespace) {
        this.namespace = namespace;
        return this;
    }

    /**
    * Get namespace
    *
     * @return namespace
     */
    @javax.annotation.Nullable
    public String getNamespace() {
        return namespace;
    }

    public void setNamespace(String namespace) {
        this.namespace = namespace;
    }

    public KvSetIn ttlMs(Long ttlMs) {
        this.ttlMs = ttlMs;
        return this;
    }

    /**
    * Time to live in milliseconds
    *
     * @return ttlMs
     */
    @javax.annotation.Nullable
    public Long getTtlMs() {
        return ttlMs;
    }

    public void setTtlMs(Long ttlMs) {
        this.ttlMs = ttlMs;
    }

    public KvSetIn behavior(OperationBehavior behavior) {
        this.behavior = behavior;
        return this;
    }

    /**
    * Get behavior
    *
     * @return behavior
     */
    @javax.annotation.Nullable
    public OperationBehavior getBehavior() {
        return behavior;
    }

    public void setBehavior(OperationBehavior behavior) {
        this.behavior = behavior;
    }

    public KvSetIn version(Long version) {
        this.version = version;
        return this;
    }

    /**
    * If set, the write only succeeds when the stored version matches this value.
Use the `version` field from a prior `get` response.
    *
     * @return version
     */
    @javax.annotation.Nullable
    public Long getVersion() {
        return version;
    }

    public void setVersion(Long version) {
        this.version = version;
    }
}