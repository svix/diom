// this file is @generated
package com.svix.coyote.models;

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
import com.svix.coyote.Utils;
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
@JsonSerialize(using = RateLimiterGetRemainingInSerializer.class)
@JsonDeserialize(using = RateLimiterGetRemainingInDeserializer.class)
public class RateLimiterGetRemainingIn {
    private String key;
    private RateLimiterGetRemainingInConfig config;

    public RateLimiterGetRemainingIn key(String key) {
        this.key = key;
        return this;
    }
    public RateLimiterGetRemainingIn config(RateLimiterGetRemainingInConfig config) {
        this.config = config;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static RateLimiterGetRemainingIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RateLimiterGetRemainingIn.class);
    }
}

@Getter
@NoArgsConstructor
class RateLimiterGetRemainingInSurrogate {
    @JsonProperty("key") String key;
    @JsonProperty("method") String method;
    @JsonProperty("config") JsonNode config;

    RateLimiterGetRemainingInSurrogate(RateLimiterGetRemainingIn o, String method, JsonNode config ){
        this.key = o.getKey();
        this.method = method;
        this.config = config;
    }
}


class RateLimiterGetRemainingInSerializer extends StdSerializer<RateLimiterGetRemainingIn> {
    public RateLimiterGetRemainingInSerializer() {
        this(null);
    }

    public RateLimiterGetRemainingInSerializer(Class<RateLimiterGetRemainingIn> t) {
        super(t);
    }

    @Override
    public void serialize(RateLimiterGetRemainingIn value, JsonGenerator gen, SerializerProvider provider) throws IOException {
        RateLimiterGetRemainingInSurrogate surrogate = new RateLimiterGetRemainingInSurrogate(value,value.getConfig().getVariantName(),value.getConfig().toJsonNode());
        gen.writeObject(surrogate);
    }
}


class RateLimiterGetRemainingInDeserializer extends StdDeserializer<RateLimiterGetRemainingIn> {
    public RateLimiterGetRemainingInDeserializer() {
        this(null);
    }

    public RateLimiterGetRemainingInDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public RateLimiterGetRemainingIn deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        RateLimiterGetRemainingInSurrogate surrogate = p.getCodec().readValue(p, RateLimiterGetRemainingInSurrogate.class);
        String key = surrogate.getKey();
        String method = surrogate.getMethod();
        JsonNode config = surrogate.getConfig();
        RateLimiterGetRemainingInConfig sourceType = RateLimiterGetRemainingInConfig.fromTypeAndConfig(method, config);
        return new RateLimiterGetRemainingIn(key,sourceType);
    }
}



