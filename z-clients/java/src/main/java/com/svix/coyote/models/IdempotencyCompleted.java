// this file is @generated
package com.svix.coyote.models;

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
import com.svix.coyote.DurationMsSerializer;
import com.svix.coyote.DurationMsDeserializer;
import com.svix.coyote.Utils;
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
public class IdempotencyCompleted {
    @JsonProperty private List<Byte> response;
    public IdempotencyCompleted() {}

    public IdempotencyCompleted response(List<Byte> response) {
        this.response = response;
        return this;
    }

    public IdempotencyCompleted addResponseItem(Byte responseItem) {
        if (this.response == null) {
            this.response = new ArrayList<>();
        }
        this.response.add(responseItem);
        return this;
    }
    /**
    * Get response
    *
     * @return response
     */
    @javax.annotation.Nonnull
    public List<Byte> getResponse() {
        return response;
    }

    public void setResponse(List<Byte> response) {
        this.response = response;
    }

    /**
     * Create an instance of IdempotencyCompleted given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IdempotencyCompleted
     * @throws JsonProcessingException if the JSON string is invalid with respect to IdempotencyCompleted
     */
    public static IdempotencyCompleted fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IdempotencyCompleted.class);
    }

    /**
     * Convert an instance of IdempotencyCompleted to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}