// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.databind.ser.std.StdSerializer;
import com.svix.diom.Utils;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;
import lombok.EqualsAndHashCode;
import lombok.AllArgsConstructor;
import lombok.NoArgsConstructor;
import java.io.IOException;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.util.HashMap;
import java.util.Map;
import java.util.List;
import java.util.Objects;
import java.net.URI;
import java.time.OffsetDateTime;

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = RateLimiterCheckInSerializer.class)
@JsonDeserialize(using = RateLimiterCheckInDeserializer.class)
public class RateLimiterCheckIn {
    private String key;
    private Long tokens;
    private RateLimiterCheckInConfig config;

    public RateLimiterCheckIn key(String key) {
        this.key = key;
        return this;
    }
    public RateLimiterCheckIn tokens(Long tokens) {
        this.tokens = tokens;
        return this;
    }
    public RateLimiterCheckIn config(RateLimiterCheckInConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static RateLimiterCheckIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimiterCheckIn.class);
    }
}

@Getter
@NoArgsConstructor
class RateLimiterCheckInSurrogate {
    @JsonProperty("key") String key;
    @JsonProperty("tokens") Long tokens;
    @JsonProperty("method") String method;
    @JsonProperty("config") JsonNode config;

    RateLimiterCheckInSurrogate(RateLimiterCheckIn o, String method, JsonNode config ){
        this.key = o.getKey();
        this.tokens = o.getTokens();
        this.method = method;
        this.config = config;
    }
}


class RateLimiterCheckInSerializer extends StdSerializer<RateLimiterCheckIn> {
    public RateLimiterCheckInSerializer() {
        this(null);
    }

    public RateLimiterCheckInSerializer(Class<RateLimiterCheckIn> t) {
        super(t);
    }

    @Override
    public void serialize(RateLimiterCheckIn value, JsonGenerator gen, SerializerProvider provider) throws IOException {
        RateLimiterCheckInSurrogate surrogate = new RateLimiterCheckInSurrogate(value,value.getConfig().getVariantName(),value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}


class RateLimiterCheckInDeserializer extends StdDeserializer<RateLimiterCheckIn> {
    public RateLimiterCheckInDeserializer() {
        this(null);
    }

    public RateLimiterCheckInDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public RateLimiterCheckIn deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        RateLimiterCheckInSurrogate surrogate = p.getCodec().readValue(p, RateLimiterCheckInSurrogate.class);
        String key = surrogate.getKey();
        Long tokens = surrogate.getTokens();
        String method = surrogate.getMethod();
        JsonNode config = surrogate.getConfig();
        RateLimiterCheckInConfig sourceType = RateLimiterCheckInConfig.fromTypeAndConfig(method, config);
        return new RateLimiterCheckIn(key,tokens,sourceType);
    }
}



