// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface AdminAuthTokenUpdateIn {
    id: string;
    name?: string | null;
    expiry?: Temporal.Duration | null;
    enabled?: boolean | null;
}

export const AdminAuthTokenUpdateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenUpdateIn {
        return {
            id: object['id'],
            name: object['name'],
            expiry: object['expiry_ms'] != null ? Temporal.Duration.from({ milliseconds: object['expiry_ms'] }) : undefined,
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenUpdateIn): any {
        return {
            'id': self.id,
            'name': self.name,
            'expiry_ms': self.expiry != null ? self.expiry.total('millisecond') : undefined,
            'enabled': self.enabled,
        };
    }
}