// this file is @generated

export interface SvixPollerDeleteIn {
    namespace?: string | null;
    topic: string;
    pollerId: string;
}

export const SvixPollerDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SvixPollerDeleteIn {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            pollerId: object['poller_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SvixPollerDeleteIn): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'poller_id': self.pollerId,
        };
    }
}