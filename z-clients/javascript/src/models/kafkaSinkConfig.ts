// this file is @generated
import {
    type KafkaSecurity,
    KafkaSecuritySerializer,
} from './kafkaSecurity';
/**
* Configuration for a Kafka sink. Each message is produced to `topic` on the target cluster. By
* default the message value and headers pass through unchanged, but each can be templated
* per-message (see [`diom_core::template_str`]).
*/
export interface KafkaSinkConfig {
    /** Comma-separated `host:port` list of the target cluster's bootstrap brokers. */
    bootstrapServers: string;
    /** Destination Kafka topic. */
    topic: string;
    /** Templated record key rendered per-message. When absent, records are produced without a key. */
    key?: string | null;
    /** Templated record value. When absent, the raw message value bytes are produced unchanged. */
    value?: string | null;
    /**
     * Templated record headers merged on top of the message's own headers (which pass through by
     * default). A templated header overrides a passed-through one with the same name.
     */
    headers?: { [key: string]: string };
    /** Connection security (SASL and/or TLS). Defaults to none (PLAINTEXT). */
    security?: KafkaSecurity;
}

export const KafkaSinkConfigSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KafkaSinkConfig {
        return {
            bootstrapServers: object['bootstrap_servers'],
            topic: object['topic'],
            key: object['key'],
            value: object['value'],
            headers: object['headers'],
            security: object['security'] != null ? KafkaSecuritySerializer._fromJsonObject(object['security']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KafkaSinkConfig): any {
        return {
            'bootstrap_servers': self.bootstrapServers,
            'topic': self.topic,
            'key': self.key,
            'value': self.value,
            'headers': self.headers,
            'security': self.security != null ? KafkaSecuritySerializer._toJsonObject(self.security) : undefined,
        };
    }
}