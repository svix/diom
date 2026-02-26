package com.svix.coyote;

import java.util.Arrays;
import java.util.List;
import lombok.Getter;
import lombok.Setter;

@Setter
@Getter
public final class CoyoteOptions {
    public static final String DEFAULT_URL = "https://api.svix.com";

    private String serverUrl;
    private final List<Long> retrySchedule = Arrays.asList(50L, 100L, 200L);
}
