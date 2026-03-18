// this file is @generated

export enum ServerState {
    Leader = 'Leader',
    Follower = 'Follower',
    Learner = 'Learner',
    Candidate = 'Candidate',
    Shutdown = 'Shutdown',
    Unknown = 'Unknown',
    }

export const ServerStateSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ServerState {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ServerState): any {
        return self;
    }
}