// this file is @generated

export interface SvixPollerCreateOut {
    topic: string;
    pollerId: string;
}

export const SvixPollerCreateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SvixPollerCreateOut {
        return {
            topic: object['topic'],
            pollerId: object['poller_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SvixPollerCreateOut): any {
        return {
            'topic': self.topic,
            'poller_id': self.pollerId,
        };
    }
}