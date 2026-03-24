// this file is @generated

export interface PingOut {
    ok: boolean;
}

export const PingOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): PingOut {
        return {
            ok: object['ok'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: PingOut): any {
        return {
            'ok': self.ok,
        };
    }
}