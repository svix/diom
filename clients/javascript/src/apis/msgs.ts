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
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export class Msgs {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get topic() {
        return new MsgsTopic(this.requestCtx);
    }

    }

