// this file is @generated

export interface MsgIn {
    value: number[];
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
    delayMs?: number | null;
}

export const MsgInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgIn {
        return {
            value: object['value'],
            headers: object['headers'],
            key: object['key'],
            delayMs: object['delay_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgIn): any {
        return {
            'value': self.value,
            'headers': self.headers,
            'key': self.key,
            'delay_ms': self.delayMs,
        };
    }
}