// this file is @generated

export interface IdempotencyCompleted {
    response: number[];
}

export const IdempotencyCompletedSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCompleted {
        return {
            response: object['response'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCompleted): any {
        return {
            'response': self.response,
        };
    }
}