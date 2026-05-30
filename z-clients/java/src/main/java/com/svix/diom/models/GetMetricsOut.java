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
public class GetMetricsOut {
    @JsonProperty private List<MetricOut> metrics;
    public GetMetricsOut() {}

    public GetMetricsOut metrics(List<MetricOut> metrics) {
        this.metrics = metrics;
        return this;
    }

    public GetMetricsOut addMetricsItem(MetricOut metricsItem) {
        if (this.metrics == null) {
            this.metrics = new ArrayList<>();
        }
        this.metrics.add(metricsItem);
        return this;
    }
    /**
    * Get metrics
    *
     * @return metrics
     */
    @javax.annotation.Nonnull
    public List<MetricOut> getMetrics() {
        return metrics;
    }

    public void setMetrics(List<MetricOut> metrics) {
        this.metrics = metrics;
    }

    /**
     * Create an instance of GetMetricsOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GetMetricsOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to GetMetricsOut
     */
    public static GetMetricsOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GetMetricsOut.class);
    }

    /**
     * Convert an instance of GetMetricsOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}