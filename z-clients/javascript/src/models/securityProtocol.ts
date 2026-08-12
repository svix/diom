// this file is @generated
/** The connection security protocol, mapped onto librdkafka's `security.protocol`. */
export enum SecurityProtocol {
    Plaintext = 'plaintext',
    Ssl = 'ssl',
    SaslPlaintext = 'sasl-plaintext',
    SaslSsl = 'sasl-ssl',
    }

export const SecurityProtocolSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SecurityProtocol {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SecurityProtocol): any {
        return self;
    }
}