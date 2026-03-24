// this file is @generated

export enum SeekPosition {
    Earliest = 'earliest',
    Latest = 'latest',
    }

export const SeekPositionSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SeekPosition {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SeekPosition): any {
        return self;
    }
}