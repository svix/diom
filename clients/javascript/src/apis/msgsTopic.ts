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
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class MsgsTopic {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Upserts a new message topic with the given name. */
        public create(
            createMsgTopicIn: CreateMsgTopicIn,
            ): Promise<CreateMsgTopicOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/topic/create");

            request.setBody(
                    CreateMsgTopicInSerializer._toJsonObject(
                        createMsgTopicIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CreateMsgTopicOutSerializer._fromJsonObject,
                );
            }

        

    /** Get message topic with given name. */
        public get(
            getMsgTopicIn: GetMsgTopicIn,
            ): Promise<GetMsgTopicOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/topic/get");

            request.setBody(
                    GetMsgTopicInSerializer._toJsonObject(
                        getMsgTopicIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    GetMsgTopicOutSerializer._fromJsonObject,
                );
            }

        

    }

