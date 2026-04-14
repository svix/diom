package com.svix.diom;

import static org.junit.jupiter.api.Assertions.*;

import com.svix.diom.models.*;
import java.time.Duration;
import java.util.Collections;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

@Tag("integration")
class IntegrationTest {

    private static final String TOKEN = requireEnv("DIOM_TOKEN");
    private static final String SERVER_URL = requireEnv("DIOM_SERVER_URL");

    private static String requireEnv(String name) {
        String value = System.getenv(name);
        if (value == null || value.isEmpty()) {
            throw new IllegalStateException(name + " must be set");
        }
        return value;
    }

    private static Diom client;

    @BeforeAll
    static void setUp() {
        DiomOptions opts = new DiomOptions();
        opts.setServerUrl(SERVER_URL);
        client = new Diom(TOKEN, opts);
    }

    @Test
    void testHealthPing() throws Exception {
        PingOut resp = client.getHealth().ping();
        assertNotNull(resp);
    }

    @Test
    void testKvSetGetDelete() throws Exception {
        String key = "java-integration-kv-key";
        byte[] value = "java-integration-kv-value".getBytes();

        // Set
        KvSetOut setResp = client.getKv().set(key, value);
        assertTrue(setResp.getSuccess());

        // Get
        KvGetOut getResp = client.getKv().get(key);
        assertArrayEquals(value, getResp.getValue());

        // Delete
        KvDeleteOut delResp = client.getKv().delete(key);
        assertTrue(delResp.getSuccess());

        // Verify deleted
        KvGetOut getResp2 = client.getKv().get(key);
        assertNull(getResp2.getValue());
    }

    @Test
    void testMsgsQueueSubresourceChaining() throws Exception {
        String namespace = "java-integration-ns";
        String topic = "java-integration-topic";
        String consumerGroup = "java-integration-cg";

        client.getMsgs().getNamespace().create(namespace);
        client.getMsgs().publish(topic, new MsgPublishIn()
            .namespace(namespace)
            .msgs(Collections.singletonList(new MsgIn().value("hello".getBytes()))));

        MsgQueueReceiveOut received = client.getMsgs().getQueue().receive(
            topic, consumerGroup, new MsgQueueReceiveIn().namespace(namespace));
        assertNotNull(received);
        assertFalse(received.getMsgs().isEmpty());

        String msgId = received.getMsgs().get(0).getMsgId();
        MsgQueueAckOut ackResp = client.getMsgs().getQueue().ack(
            topic, consumerGroup,
            new MsgQueueAckIn()
                .namespace(namespace)
                .msgIds(Collections.singletonList(msgId)));
        assertNotNull(ackResp);
    }

    @Test
    void testCacheSetGetDelete() throws Exception {
        String key = "java-integration-cache-key";
        byte[] value = "java-integration-cache-value".getBytes();

        // Set
        client.getCache().set(key, value, new CacheSetIn().ttl(Duration.ofMillis(60000)));

        // Get
        CacheGetOut getResp = client.getCache().get(key);
        assertArrayEquals(value, getResp.getValue());

        // Delete
        CacheDeleteOut delResp = client.getCache().delete(key);
        assertTrue(delResp.getSuccess());

        // Verify deleted
        CacheGetOut getResp2 = client.getCache().get(key);
        assertNull(getResp2.getValue());
    }
}
