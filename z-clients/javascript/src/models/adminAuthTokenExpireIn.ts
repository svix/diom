// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface AdminAuthTokenExpireIn {
    id: string;
    /** Milliseconds from now until the token expires. `None` means expire immediately. */
    expiry?: Temporal.Duration | null;
}

export const AdminAuthTokenExpireInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenExpireIn {
        return {
            id: object['id'],
            expiry: object['expiry_ms'] != null ? Temporal.Duration.from({ milliseconds: object['expiry_ms'] }) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenExpireIn): any {
        return {
            'id': self.id,
            'expiry_ms': self.expiry != null ? self.expiry.total('millisecond') : undefined,
        };
    }
}