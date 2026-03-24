// this file is @generated

export interface MsgQueueRedriveDlqIn {
    namespace?: string | null;
}

export interface MsgQueueRedriveDlqIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
}

export const MsgQueueRedriveDlqInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueRedriveDlqIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueRedriveDlqIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
        };
    }
}