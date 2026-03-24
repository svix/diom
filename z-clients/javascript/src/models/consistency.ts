// this file is @generated
/**
* Consistency level for reads.
* 
* Strong consistency (also known as linearizability) guarantees that a read will see all previous
* writes. Weak consistency allows stale reads, but can save one or more round trip to the leader.
*/
export enum Consistency {
    Strong = 'strong',
    Weak = 'weak',
    }

export const ConsistencySerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): Consistency {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: Consistency): any {
        return self;
    }
}