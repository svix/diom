// this file is @generated

export interface MsgTopicConfigureIn {
    namespace?: string | null;
    partitions: number;
}

export interface MsgTopicConfigureIn_ {
    namespace?: string | null;
    topic: string;
    partitions: number;
}

export const MsgTopicConfigureInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicConfigureIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            partitions: object['partitions'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicConfigureIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'partitions': self.partitions,
        };
    }
}