// this file is @generated

export interface TransformIn {
    /** JSON-encoded payload passed to the script as `input`. */
    input: string;
    /** JavaScript source. Must define a `handler(input)` function. */
    script: string;
    /** How long to let the script run before being killed. */
    maxDurationMs?: number;
}

export const TransformInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): TransformIn {
        return {
            input: object['input'],
            script: object['script'],
            maxDurationMs: object['max_duration_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: TransformIn): any {
        return {
            'input': self.input,
            'script': self.script,
            'max_duration_ms': self.maxDurationMs,
        };
    }
}