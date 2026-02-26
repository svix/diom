// this file is @generated

import {
    type CreateMsgTopicIn,
    CreateMsgTopicInSerializer,
} from '../models/createMsgTopicIn';
import {
    type CreateMsgTopicOut,
    CreateMsgTopicOutSerializer,
} from '../models/createMsgTopicOut';
import {
    type GetMsgTopicIn,
    GetMsgTopicInSerializer,
} from '../models/getMsgTopicIn';
import {
    type GetMsgTopicOut,
    GetMsgTopicOutSerializer,
} from '../models/getMsgTopicOut';
import { MsgsTopic } from './msgsTopic';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class Msgs {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    public get topic() {
        return new MsgsTopic(this.requestCtx);
    }

    }

