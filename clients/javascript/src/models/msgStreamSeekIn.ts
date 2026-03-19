// this file is @generated

export interface MsgStreamSeekIn {
    namespace?: string | null;
    offset?: number | null;
    position?: string | null;
}

export interface MsgStreamSeekIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    offset?: number | null;
    position?: string | null;
}

export const MsgStreamSeekInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgStreamSeekIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            offset: object['offset'],
            position: object['position'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgStreamSeekIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'offset': self.offset,
            'position': self.position,
        };
    }
}