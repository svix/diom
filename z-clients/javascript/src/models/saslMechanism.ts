// this file is @generated
/** The SASL mechanism, mapped onto librdkafka's `sasl.mechanism`. */
export enum SaslMechanism {
    Plain = 'plain',
    ScramSha256 = 'scram-sha256',
    ScramSha512 = 'scram-sha512',
    }

export const SaslMechanismSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SaslMechanism {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SaslMechanism): any {
        return self;
    }
}