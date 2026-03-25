// this file is @generated

export interface TransformOut {
    /** JSON-encoded value returned by the script's `handler` function. */
    output: string;
}

export const TransformOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): TransformOut {
        return {
            output: object['output'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: TransformOut): any {
        return {
            'output': self.output,
        };
    }
}