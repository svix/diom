package com.svix.diom;

import static com.github.tomakehurst.wiremock.client.WireMock.*;
import static org.junit.jupiter.api.Assertions.*;

import com.github.tomakehurst.wiremock.WireMockServer;
import com.github.tomakehurst.wiremock.core.WireMockConfiguration;
import com.github.tomakehurst.wiremock.verification.LoggedRequest;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class WiremockTests {
    private WireMockServer server;

    @BeforeEach
    void setUp() {
        server = new WireMockServer(WireMockConfiguration.wireMockConfig().dynamicPort());
        server.start();
    }

    @AfterEach
    void tearDown() {
        server.stop();
    }

    @Test
    void testRetrySchedule() throws Exception {
        Map<String, String> error = new HashMap<>();
        error.put("type", "server-error");
        error.put("code", "test_error");
        error.put("detail", "test detail");
        server.stubFor(
                get(urlEqualTo("/api/v1.health.ping"))
                        .willReturn(
                                aResponse()
                                        .withStatus(500)
                                        .withBody(
                                                Utils.getMsgpackObjectMapper()
                                                        .writeValueAsBytes(error))));

        DiomOptions opts = new DiomOptions();
        opts.setServerUrl(server.baseUrl());
        opts.setRetrySchedule(Arrays.asList(1L, 1L, 1L));
        Diom client = new Diom("test-token", opts);

        assertThrows(ServerErrorException.class, () -> client.health().ping());

        List<LoggedRequest> requests =
                server.findAll(getRequestedFor(urlEqualTo("/api/v1.health.ping")));
        assertEquals(4, requests.size());
        assertNull(requests.get(0).getHeader("diom-retry-count"));
        String reqId = requests.get(0).getHeader("diom-req-id");
        for (int i = 1; i < requests.size(); i++) {
            assertEquals(String.valueOf(i), requests.get(i).getHeader("diom-retry-count"));
            assertEquals(reqId, requests.get(i).getHeader("diom-req-id"));
        }
    }
}
