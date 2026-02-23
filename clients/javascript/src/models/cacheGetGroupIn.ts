// this file is @generated





export interface CacheGetGroupIn {
    name: string;
}

export const CacheGetGroupInSerializer = {
    _fromJsonObject(object: any): CacheGetGroupIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: CacheGetGroupIn): any {
        return {
            'name': self.name,
            };
    }
}