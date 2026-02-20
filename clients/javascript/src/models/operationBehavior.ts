// this file is @generated

export enum OperationBehavior {
    Upsert = 'upsert',
    Insert = 'insert',
    Update = 'update',
    }

export const OperationBehaviorSerializer = {
    _fromJsonObject(object: any): OperationBehavior {
        return object;
    },

    _toJsonObject(self: OperationBehavior): any {
        return self;
    }
}