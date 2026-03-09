// this file is @generated

export enum OperationBehavior {
    Upsert = 'upsert',
    Insert = 'insert',
    Update = 'update',
    }

export const OperationBehaviorSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): OperationBehavior {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: OperationBehavior): any {
        return self;
    }
}