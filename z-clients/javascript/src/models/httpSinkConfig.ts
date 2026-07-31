// this file is @generated
import {
    type HttpMethod,
    HttpMethodSerializer,
} from './httpMethod';
/**
* Configuration for an HTTP sink. The `url`, `headers`, and `body` are templates rendered
* per-message (see [`diom_core::template_str`]).
*/
export interface HttpSinkConfig {
    /** Destination URL. */
    url: string;
    method?: HttpMethod;
    headers?: { [key: string]: string };
    /** Templated request body. When absent, the raw message value bytes are sent unchanged. */
    body?: string | null;
}

export const HttpSinkConfigSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): HttpSinkConfig {
        return {
            url: object['url'],
            method: object['method'] != null ? HttpMethodSerializer._fromJsonObject(object['method']): undefined,
            headers: object['headers'],
            body: object['body'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: HttpSinkConfig): any {
        return {
            'url': self.url,
            'method': self.method != null ? HttpMethodSerializer._toJsonObject(self.method) : undefined,
            'headers': self.headers,
            'body': self.body,
        };
    }
}