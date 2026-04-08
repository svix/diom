package com.svix.coyote;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import java.util.List;
import java.util.Set;

public class Utils {
    private static boolean isEnum(Object v) {
        return v != null && v.getClass().isEnum();
    }

    private static boolean isList(Object v) {
        return v instanceof List;
    }

    private static boolean isSet(Object v) {
        return v instanceof Set;
    }

    public static ObjectMapper getObjectMapper() {
        ObjectMapper mapper = new ObjectMapper();
        mapper.enable(JsonParser.Feature.INCLUDE_SOURCE_IN_LOCATION);
        mapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        mapper.registerModule(new Jdk8Module());
        return mapper;
    }
}
