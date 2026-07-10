// this file is @generated

export interface SvixPollerOut {
    topic: string;
    pollerId: string;
    /** The autoconfig token, obfuscated (e.g. `auto_v1_eyJh...fQ==`). */
    token: string;
}

export const SvixPollerOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SvixPollerOut {
        return {
            topic: object['topic'],
            pollerId: object['poller_id'],
            token: object['token'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SvixPollerOut): any {
        return {
            'topic': self.topic,
            'poller_id': self.pollerId,
            'token': self.token,
        };
    }
}