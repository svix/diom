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
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.svix.diom.DurationMsSerializer;
import com.svix.diom.DurationMsDeserializer;
import com.svix.diom.Utils;
import java.time.Duration;
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
public class KvGetIn {
    @JsonProperty private String namespace;
    @JsonProperty private Consistency consistency;
    @JsonProperty("use_postgres") private Boolean usePostgres;
    public KvGetIn() {}

    public KvGetIn namespace(String namespace) {
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

    public KvGetIn consistency(Consistency consistency) {
        this.consistency = consistency;
        return this;
    }

    /**
    * Get consistency
    *
     * @return consistency
     */
    @javax.annotation.Nullable
    public Consistency getConsistency() {
        return consistency;
    }

    public void setConsistency(Consistency consistency) {
        this.consistency = consistency;
    }

    public KvGetIn usePostgres(Boolean usePostgres) {
        this.usePostgres = usePostgres;
        return this;
    }

    /**
    * If true, fetch from postgres instead of fjall (for benchmarking).
    *
     * @return usePostgres
     */
    @javax.annotation.Nullable
    public Boolean getUsePostgres() {
        return usePostgres;
    }

    public void setUsePostgres(Boolean usePostgres) {
        this.usePostgres = usePostgres;
    }
}