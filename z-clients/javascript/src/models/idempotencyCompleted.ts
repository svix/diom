// this file is @generated

export interface IdempotencyCompleted {
    response: Uint8Array;
    context?: { [key: string]: string } | null;
}

export const IdempotencyCompletedSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCompleted {
        return {
            response: new Uint8Array(object['response']),
            context: object['context'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCompleted): any {
        return {
            'response': Array.from(self.response),
            'context': self.context,
        };
    }
}