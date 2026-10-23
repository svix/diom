import { describe, it, beforeEach, afterEach } from "node:test";
import assert from "node:assert/strict";
import { Diom, ServerError, InvalidInputError } from "../dist/index.mjs";
import { Temporal } from "temporal-polyfill-lite";
import * as mockttp from "mockttp";

const TOKEN = process.env.DIOM_TOKEN;
const SERVER_URL = process.env.DIOM_SERVER_URL;

const mockServer = mockttp.getLocal();

describe("SDK Client Tests", () => {
  beforeEach(async () => await mockServer.start(0));
  afterEach(async () => await mockServer.stop());

  it("test_health_ping", async () => {
    const mock = await mockServer
      .forGet("/api/v1.health.ping")
      .thenReply(200, JSON.stringify({ ok: true }), {
        "Content-Type": "application/json",
      });
    const client = new Diom("token", {
      serverUrl: mockServer.url,
    });
    const resp = await client.health.ping();
    assert.ok(resp);
    assert.equal((await mock.getSeenRequests()).length, 1);
  });

  it("retries on 5xx", async () => {
    const failMock = await mockServer
      .forGet("/api/v1.health.ping")
      .once()
      .thenReply(500, "INTERNAL SERVER ERROR", {
        "Content-Type": "text/plain",
      });
    const okMock = await mockServer
      .forGet("/api/v1.health.ping")
      .thenReply(200, JSON.stringify({ ok: true }), {
        "Content-Type": "application/json",
      });
    const client = new Diom("token", {
      serverUrl: mockServer.url,
      retryScheduleInMs: [10, 10],
    });
    const resp = await client.health.ping();
    assert.ok(resp);
    assert(resp.ok);
    assert.equal((await failMock.getSeenRequests()).length, 1);
    assert.equal((await okMock.getSeenRequests()).length, 1);
  });

  it("retries on timeout", async () => {
    const failMock = await mockServer
      .forGet("/api/v1.health.ping")
      .once()
      .thenTimeout();
    const okMock = await mockServer
      .forGet("/api/v1.health.ping")
      .thenReply(200, JSON.stringify({ ok: true }), {
        "Content-Type": "application/json",
      });
    const client = new Diom("token", {
      serverUrl: mockServer.url,
      requestTimeout: 50,
      retryScheduleInMs: [10, 10],
    });
    const resp = await client.health.ping();
    assert.ok(resp);
    assert(resp.ok);
    assert.equal((await failMock.getSeenRequests()).length, 1);
    assert.equal((await okMock.getSeenRequests()).length, 1);
  });

  it("does not retry on 4xx", async () => {
    const failMock = await mockServer
      .forGet("/api/v1.health.ping")
      .once()
      .thenReply(
        400,
        JSON.stringify({
          type: "invalid-input",
          code: "BAD_REQUEST",
          detail: "naughty, dirty little request",
        }),
        { "Content-Type": "application/json" },
      );
    const client = new Diom("token", {
      serverUrl: mockServer.url,
      requestTimeout: 50,
      retryScheduleInMs: [10, 10],
    });
    await assert.rejects(async () => {
      await client.health.ping();
    }, InvalidInputError);
    assert.equal((await failMock.getSeenRequests()).length, 1);
  });
});
