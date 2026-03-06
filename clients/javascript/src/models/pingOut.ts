// this file is @generated

export interface PingOut {
    ok: boolean;
}

export const PingOutSerializer = {
    _fromJsonObject(object: any): PingOut {
        return {
            ok: object['ok'],
        };
    },

    _toJsonObject(self: PingOut): any {
        return {
            'ok': self.ok,
        };
    }
}