// this file is @generated

export interface MsgFifoRedriveDlqIn {
    namespace?: string | null;
}

export interface MsgFifoRedriveDlqIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
}

export const MsgFifoRedriveDlqInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgFifoRedriveDlqIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgFifoRedriveDlqIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
        };
    }
}