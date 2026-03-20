// this file is @generated
import {
    type AuthTokenOut,
    AuthTokenOutSerializer,
} from './authTokenOut';

export interface AuthTokenVerifyOut {
    token?: AuthTokenOut | null;
}

export const AuthTokenVerifyOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenVerifyOut {
        return {
            token: object['token'] != null ? AuthTokenOutSerializer._fromJsonObject(object['token']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenVerifyOut): any {
        return {
            'token': self.token != null ? AuthTokenOutSerializer._toJsonObject(self.token) : undefined,
        };
    }
}