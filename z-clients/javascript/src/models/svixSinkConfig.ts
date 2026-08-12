// this file is @generated
/**
* Configuration for a Svix sink. Each message is forwarded as a Svix message-create call
* (`POST {server_url}/api/v1/app/{app_id}/msg/`). This is a thin convenience over an HTTP sink.
* The `app_id`, `event_type`, and `payload` are templates rendered per-message
* (see [`diom_core::template_str`]).
*/
export interface SvixSinkConfig {
    /** Svix API token, sent as the bearer credential. Obfuscated in list responses. */
    token: string;
    /** Target Svix application. Can be optionally templated. */
    appId: string;
    /** Svix event type. Can be optionally templated. */
    eventType: string;
    /** Templated message payload. When absent, the raw message value bytes are used (must be JSON). */
    payload?: string | null;
    /**
     * Templated Svix `Idempotency-Key`. When absent or it renders to an empty string, a stable
     * key derived from the sink and message identity (namespace, topic, consumer_group, partition,
     * offset) is used so retries are de-duplicated by Svix.
     */
    idempotencyKey?: string | null;
    /** Optional base URL override. When absent, the region is inferred from the token. */
    serverUrl?: string | null;
}

export const SvixSinkConfigSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SvixSinkConfig {
        return {
            token: object['token'],
            appId: object['app_id'],
            eventType: object['event_type'],
            payload: object['payload'],
            idempotencyKey: object['idempotency_key'],
            serverUrl: object['server_url'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SvixSinkConfig): any {
        return {
            'token': self.token,
            'app_id': self.appId,
            'event_type': self.eventType,
            'payload': self.payload,
            'idempotency_key': self.idempotencyKey,
            'server_url': self.serverUrl,
        };
    }
}