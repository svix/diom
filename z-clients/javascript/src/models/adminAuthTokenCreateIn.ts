// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface AdminAuthTokenCreateIn {
    name: string;
    role: string;
    /** Milliseconds from now until the token expires. */
    expiry?: Temporal.Duration | null;
    /** Whether the token is enabled. Defaults to `true`. */
    enabled?: boolean;
}

export const AdminAuthTokenCreateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenCreateIn {
        return {
            name: object['name'],
            role: object['role'],
            expiry: object['expiry_ms'] != null ? Temporal.Duration.from({ milliseconds: object['expiry_ms'] }) : undefined,
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenCreateIn): any {
        return {
            'name': self.name,
            'role': self.role,
            'expiry_ms': self.expiry != null ? self.expiry.total('millisecond') : undefined,
            'enabled': self.enabled,
        };
    }
}