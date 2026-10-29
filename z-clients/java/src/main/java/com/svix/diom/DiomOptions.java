package com.svix.diom;

import java.util.Arrays;
import java.util.List;
import lombok.Getter;
import lombok.Setter;

@Setter
@Getter
public final class DiomOptions {
    public static final String DEFAULT_URL = "https://api.svix.com";

    private String serverUrl;
    /**
     * Delay, in milliseconds, to wait before each retry of a failed request.
     *
     * <p>The list has one entry per retry. Defaults to no retries.
     */
    private List<Long> retrySchedule = Arrays.asList();
}
