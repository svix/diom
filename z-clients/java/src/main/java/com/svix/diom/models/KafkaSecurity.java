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
import com.svix.diom.UnixTimestampMsSerializer;
import com.svix.diom.UnixTimestampMsDeserializer;
import com.svix.diom.Utils;
import java.time.Duration;
import java.time.Instant;
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
public class KafkaSecurity {
    @JsonProperty("security_protocol") private SecurityProtocol securityProtocol;
    @JsonProperty("sasl_mechanism") private SaslMechanism saslMechanism;
    @JsonProperty("sasl_username") private String saslUsername;
    @JsonProperty("sasl_password") private String saslPassword;
    @JsonProperty("ssl_ca_pem") private String sslCaPem;
    @JsonProperty("ssl_certificate_pem") private String sslCertificatePem;
    @JsonProperty("ssl_key_pem") private String sslKeyPem;
    @JsonProperty("ssl_key_password") private String sslKeyPassword;
    @JsonProperty("enable_ssl_certificate_verification") private Boolean enableSslCertificateVerification;
    public KafkaSecurity() {}

    public KafkaSecurity securityProtocol(SecurityProtocol securityProtocol) {
        this.securityProtocol = securityProtocol;
        return this;
    }

    /**
    * Get securityProtocol
    *
     * @return securityProtocol
     */
    @javax.annotation.Nullable
    public SecurityProtocol getSecurityProtocol() {
        return securityProtocol;
    }

    public void setSecurityProtocol(SecurityProtocol securityProtocol) {
        this.securityProtocol = securityProtocol;
    }

    public KafkaSecurity saslMechanism(SaslMechanism saslMechanism) {
        this.saslMechanism = saslMechanism;
        return this;
    }

    /**
    * Get saslMechanism
    *
     * @return saslMechanism
     */
    @javax.annotation.Nullable
    public SaslMechanism getSaslMechanism() {
        return saslMechanism;
    }

    public void setSaslMechanism(SaslMechanism saslMechanism) {
        this.saslMechanism = saslMechanism;
    }

    public KafkaSecurity saslUsername(String saslUsername) {
        this.saslUsername = saslUsername;
        return this;
    }

    /**
    * Get saslUsername
    *
     * @return saslUsername
     */
    @javax.annotation.Nullable
    public String getSaslUsername() {
        return saslUsername;
    }

    public void setSaslUsername(String saslUsername) {
        this.saslUsername = saslUsername;
    }

    public KafkaSecurity saslPassword(String saslPassword) {
        this.saslPassword = saslPassword;
        return this;
    }

    /**
    * Secret. Obfuscated in list responses.
    *
     * @return saslPassword
     */
    @javax.annotation.Nullable
    public String getSaslPassword() {
        return saslPassword;
    }

    public void setSaslPassword(String saslPassword) {
        this.saslPassword = saslPassword;
    }

    public KafkaSecurity sslCaPem(String sslCaPem) {
        this.sslCaPem = sslCaPem;
        return this;
    }

    /**
    * Inline CA certificate PEM. When absent, the system trust roots are used.
    *
     * @return sslCaPem
     */
    @javax.annotation.Nullable
    public String getSslCaPem() {
        return sslCaPem;
    }

    public void setSslCaPem(String sslCaPem) {
        this.sslCaPem = sslCaPem;
    }

    public KafkaSecurity sslCertificatePem(String sslCertificatePem) {
        this.sslCertificatePem = sslCertificatePem;
        return this;
    }

    /**
    * Inline client certificate PEM for mutual TLS.
    *
     * @return sslCertificatePem
     */
    @javax.annotation.Nullable
    public String getSslCertificatePem() {
        return sslCertificatePem;
    }

    public void setSslCertificatePem(String sslCertificatePem) {
        this.sslCertificatePem = sslCertificatePem;
    }

    public KafkaSecurity sslKeyPem(String sslKeyPem) {
        this.sslKeyPem = sslKeyPem;
        return this;
    }

    /**
    * Inline client key PEM for mutual TLS. Secret. Fully redacted in list responses.
    *
     * @return sslKeyPem
     */
    @javax.annotation.Nullable
    public String getSslKeyPem() {
        return sslKeyPem;
    }

    public void setSslKeyPem(String sslKeyPem) {
        this.sslKeyPem = sslKeyPem;
    }

    public KafkaSecurity sslKeyPassword(String sslKeyPassword) {
        this.sslKeyPassword = sslKeyPassword;
        return this;
    }

    /**
    * Password for an encrypted client key. Secret. Fully redacted in list responses.
    *
     * @return sslKeyPassword
     */
    @javax.annotation.Nullable
    public String getSslKeyPassword() {
        return sslKeyPassword;
    }

    public void setSslKeyPassword(String sslKeyPassword) {
        this.sslKeyPassword = sslKeyPassword;
    }

    public KafkaSecurity enableSslCertificateVerification(Boolean enableSslCertificateVerification) {
        this.enableSslCertificateVerification = enableSslCertificateVerification;
        return this;
    }

    /**
    * Get enableSslCertificateVerification
    *
     * @return enableSslCertificateVerification
     */
    @javax.annotation.Nullable
    public Boolean getEnableSslCertificateVerification() {
        return enableSslCertificateVerification;
    }

    public void setEnableSslCertificateVerification(Boolean enableSslCertificateVerification) {
        this.enableSslCertificateVerification = enableSslCertificateVerification;
    }

    /**
     * Create an instance of KafkaSecurity given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of KafkaSecurity
     * @throws JsonProcessingException if the JSON string is invalid with respect to KafkaSecurity
     */
    public static KafkaSecurity fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, KafkaSecurity.class);
    }

    /**
     * Convert an instance of KafkaSecurity to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}