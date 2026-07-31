// this file is @generated

export interface SinkListIn {
    namespace?: string | null;
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export interface SinkListIn_ {
    namespace?: string | null;
    topic: string;
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export const SinkListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SinkListIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            limit: object['limit'],
            iterator: object['iterator'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SinkListIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'limit': self.limit,
            'iterator': self.iterator,
        };
    }
}