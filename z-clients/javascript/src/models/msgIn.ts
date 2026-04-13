// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface MsgIn {
    value: Uint8Array;
    headers?: { [key: string]: string };
    /**
     * Optional partition key.
     * 
     * Messages with the same key are routed to the same partition.
     */
    key?: string | null;
    /**
     * Optional delay in milliseconds.
     * 
     * The message will not be delivered to queue consumers
     * until the delay has elapsed from the time of publish.
     */
    delay?: Temporal.Duration | null;
}

export const MsgInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgIn {
        return {
            value: new Uint8Array(object['value']),
            headers: object['headers'],
            key: object['key'],
            delay: object['delay_ms'] != null ? Temporal.Duration.from({ milliseconds: object['delay_ms'] }) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgIn): any {
        return {
            'value': Array.from(self.value),
            'headers': self.headers,
            'key': self.key,
            'delay_ms': self.delay != null ? self.delay.total('millisecond') : undefined,
        };
    }
}