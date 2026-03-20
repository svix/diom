// this file is @generated

export interface AuthTokenDeleteOut {
    success: boolean;
}

export const AuthTokenDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenDeleteOut {
        return {
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenDeleteOut): any {
        return {
            'success': self.success,
        };
    }
}