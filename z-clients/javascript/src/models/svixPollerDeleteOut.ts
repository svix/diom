// this file is @generated

export interface SvixPollerDeleteOut {
    topic: string;
    pollerId: string;
    success: boolean;
}

export const SvixPollerDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SvixPollerDeleteOut {
        return {
            topic: object['topic'],
            pollerId: object['poller_id'],
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SvixPollerDeleteOut): any {
        return {
            'topic': self.topic,
            'poller_id': self.pollerId,
            'success': self.success,
        };
    }
}