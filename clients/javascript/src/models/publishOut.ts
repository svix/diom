// this file is @generated
import {
    type PublishOutMsg,
    PublishOutMsgSerializer,
} from './publishOutMsg';





export interface PublishOut {
    msgs: PublishOutMsg[];
}

export const PublishOutSerializer = {
    _fromJsonObject(object: any): PublishOut {
        return {
            msgs: object['msgs'].map((item: PublishOutMsg) => PublishOutMsgSerializer._fromJsonObject(item)),
            };
    },

    _toJsonObject(self: PublishOut): any {
        return {
            'msgs': self.msgs.map((item) => PublishOutMsgSerializer._toJsonObject(item)),
            };
    }
}