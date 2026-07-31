// this file is @generated

export enum HttpMethod {
    Post = 'post',
    Put = 'put',
    Patch = 'patch',
    }

export const HttpMethodSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): HttpMethod {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: HttpMethod): any {
        return self;
    }
}