// this file is @generated

export interface ReadyOut {
    ok: boolean;
    message: string;
}

export const ReadyOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ReadyOut {
        return {
            ok: object['ok'],
            message: object['message'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ReadyOut): any {
        return {
            'ok': self.ok,
            'message': self.message,
        };
    }
}