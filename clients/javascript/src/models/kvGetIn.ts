// this file is @generated

export interface KvGetIn {
    key: string;
    /**
* Whether or not the read should be linearizable
* 
* If this is `true`, the read is guaranteed to see all previous operations, but will
* have to make at least one additional round-trip to the leader. If this is false, stale
* reads will be performed against the replica which receives this request.
*/
    linearizable?: boolean;
}

export const KvGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetIn {
        return {
            key: object['key'],
            linearizable: object['linearizable'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetIn): any {
        return {
            'key': self.key,
            'linearizable': self.linearizable,
        };
    }
}