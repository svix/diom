// this file is @generated
import {
    type MsgIn,
    MsgInSerializer,
} from './msgIn';





export interface AppendToStreamIn {
    msgs: MsgIn[];
name: string;
}

export const AppendToStreamInSerializer = {
    _fromJsonObject(object: any): AppendToStreamIn {
        return {
            msgs: object['msgs'].map((item: MsgIn) => MsgInSerializer._fromJsonObject(item)),
            name: object['name'],
            };
    },

    _toJsonObject(self: AppendToStreamIn): any {
        return {
            'msgs': self.msgs.map((item) => MsgInSerializer._toJsonObject(item)),
            'name': self.name,
            };
    }
}