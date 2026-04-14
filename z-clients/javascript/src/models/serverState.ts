// this file is @generated

export enum ServerState {
    Leader = 'leader',
    Follower = 'follower',
    Learner = 'learner',
    Candidate = 'candidate',
    Shutdown = 'shutdown',
    Unknown = 'unknown',
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