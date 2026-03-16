// this file is @generated
import {
    type IdempotencyCompleted,
    IdempotencyCompletedSerializer,
} from './idempotencyCompleted';
interface _IdempotencyStartOutFields {}


    
// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface IdempotencyStartOutStartedData {}
    

    
// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface IdempotencyStartOutLockedData {}
    

    



interface IdempotencyStartOutStarted {
    status: 'started';
    data?: IdempotencyStartOutStartedData
}

interface IdempotencyStartOutLocked {
    status: 'locked';
    data?: IdempotencyStartOutLockedData
}

interface IdempotencyStartOutCompleted {
    status: 'completed';
    data: IdempotencyCompleted;
    
}



export type IdempotencyStartOut = _IdempotencyStartOutFields & (| IdempotencyStartOutStarted
    | IdempotencyStartOutLocked
    | IdempotencyStartOutCompleted
    );

export const IdempotencyStartOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyStartOut {
        const status = object['status'];

        // biome-ignore lint/suspicious/noExplicitAny: intentional any
        function getData(status: string): any {
            switch (status) {
                
                case 'started':
                    return {}
                
                case 'locked':
                    return {}
                case 'completed':
                    return IdempotencyCompletedSerializer._fromJsonObject(
                            object['data']
                        );default:
                    throw new Error(`Unexpected status: ${ status }`);
            }
        }

        return {
            status,
            data:getData(status),
            };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyStartOut): any {
        // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
        let data;
        switch (self.status) {
            case 'started':
                data = {}
                break;
            case 'locked':
                data = {}
                break;
            case 'completed':
                data =
                    IdempotencyCompletedSerializer._toJsonObject(
                        self.data
                    );
                break;}

        return {
            'status': self.status,
            'data': data,
            };
    }
}