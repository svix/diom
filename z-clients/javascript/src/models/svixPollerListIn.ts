// this file is @generated

export interface SvixPollerListIn {
    namespace?: string | null;
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export interface SvixPollerListIn_ {
    namespace?: string | null;
    topic: string;
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export const SvixPollerListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SvixPollerListIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            limit: object['limit'],
            iterator: object['iterator'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SvixPollerListIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'limit': self.limit,
            'iterator': self.iterator,
        };
    }
}