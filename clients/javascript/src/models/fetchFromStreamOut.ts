// this file is @generated
import {
    type MsgOut,
    MsgOutSerializer,
} from './msgOut';





export interface FetchFromStreamOut {
    msgs: MsgOut[];
}

export const FetchFromStreamOutSerializer = {
    _fromJsonObject(object: any): FetchFromStreamOut {
        return {
            msgs: object['msgs'].map((item: MsgOut) => MsgOutSerializer._fromJsonObject(item)),
            };
    },

    _toJsonObject(self: FetchFromStreamOut): any {
        return {
            'msgs': self.msgs.map((item) => MsgOutSerializer._toJsonObject(item)),
            };
    }
}