// this file is @generated
import {
    type MsgPublishOutTopic,
    MsgPublishOutTopicSerializer,
} from './msgPublishOutTopic';

export interface MsgPublishOut {
    topics: MsgPublishOutTopic[];
}

export const MsgPublishOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgPublishOut {
        return {
            topics: object['topics'].map((item: MsgPublishOutTopic) => MsgPublishOutTopicSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgPublishOut): any {
        return {
            'topics': self.topics.map((item) => MsgPublishOutTopicSerializer._toJsonObject(item)),
        };
    }
}