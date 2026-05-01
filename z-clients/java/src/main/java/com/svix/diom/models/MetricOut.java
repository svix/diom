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
public class MetricOut {
    @JsonProperty private String label;
    @JsonProperty private String description;
    @JsonProperty private Map<String, String> attributes;
    @JsonProperty private Double value;
    @JsonProperty("metric_type") private MetricType metricType;
    @JsonProperty @JsonSerialize(using = UnixTimestampMsSerializer.class) @JsonDeserialize(using = UnixTimestampMsDeserializer.class) private Instant timestamp;
    @JsonProperty private String unit;
    public MetricOut() {}

    public MetricOut label(String label) {
        this.label = label;
        return this;
    }

    /**
    * Label for this series
    *
     * @return label
     */
    @javax.annotation.Nonnull
    public String getLabel() {
        return label;
    }

    public void setLabel(String label) {
        this.label = label;
    }

    public MetricOut description(String description) {
        this.description = description;
        return this;
    }

    /**
    * Human-readable description of this series
    *
     * @return description
     */
    @javax.annotation.Nonnull
    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public MetricOut attributes(Map<String, String> attributes) {
        this.attributes = attributes;
        return this;
    }

    public MetricOut putAttributesItem(String key, String attributesItem) {
        if (this.attributes == null) {
            this.attributes = new HashMap<>();
        }
        this.attributes.put(key, attributesItem);
        return this;
    }
    /**
    * Key/Value pairs attached to this sequence
    *
     * @return attributes
     */
    @javax.annotation.Nonnull
    public Map<String, String> getAttributes() {
        return attributes;
    }

    public void setAttributes(Map<String, String> attributes) {
        this.attributes = attributes;
    }

    public MetricOut value(Double value) {
        this.value = value;
        return this;
    }

    /**
    * Most recent data point for this series

All points (u64, i64, and f64) are squished into an f64, be careful
of inexactness for values above 2**53.
    *
     * @return value
     */
    @javax.annotation.Nonnull
    public Double getValue() {
        return value;
    }

    public void setValue(Double value) {
        this.value = value;
    }

    public MetricOut metricType(MetricType metricType) {
        this.metricType = metricType;
        return this;
    }

    /**
    * Type of this metric

Histograms are not currently exported through this API, and can
only be accessed through OTLP.
    *
     * @return metricType
     */
    @javax.annotation.Nonnull
    public MetricType getMetricType() {
        return metricType;
    }

    public void setMetricType(MetricType metricType) {
        this.metricType = metricType;
    }

    public MetricOut timestamp(Instant timestamp) {
        this.timestamp = timestamp;
        return this;
    }

    /**
    * Timestamp this metric was collected
    *
     * @return timestamp
     */
    @javax.annotation.Nonnull
    public Instant getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Instant timestamp) {
        this.timestamp = timestamp;
    }

    public MetricOut unit(String unit) {
        this.unit = unit;
        return this;
    }

    /**
    * Optional unit, following UCUM unit conventions if possible

See https://ucum.org/ for details
    *
     * @return unit
     */
    @javax.annotation.Nullable
    public String getUnit() {
        return unit;
    }

    public void setUnit(String unit) {
        this.unit = unit;
    }

    /**
     * Create an instance of MetricOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MetricOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to MetricOut
     */
    public static MetricOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MetricOut.class);
    }

    /**
     * Convert an instance of MetricOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}