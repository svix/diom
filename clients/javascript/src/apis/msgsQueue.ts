// this file is @generated

import {
    type MsgQueueAckIn,
    MsgQueueAckInSerializer,
} from '../models/msgQueueAckIn';
import {
    type MsgQueueAckOut,
    MsgQueueAckOutSerializer,
} from '../models/msgQueueAckOut';
import {
    type MsgQueueReceiveIn,
    MsgQueueReceiveInSerializer,
} from '../models/msgQueueReceiveIn';
import {
    type MsgQueueReceiveOut,
    MsgQueueReceiveOutSerializer,
} from '../models/msgQueueReceiveOut';
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export class MsgsQueue {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Receives messages from a topic as competing consumers.
* 
* Messages are individually leased for the specified duration. Multiple consumers can receive
* different messages from the same topic concurrently. Leased messages are skipped until they
* are acked or their lease expires.
*/
        public receive(
            msgQueueReceiveIn: MsgQueueReceiveIn,
            ): Promise<MsgQueueReceiveOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/queue/receive");

            request.setBody(
                    MsgQueueReceiveInSerializer._toJsonObject(
                        msgQueueReceiveIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    MsgQueueReceiveOutSerializer._fromJsonObject,
                );
            }

        

    /**
* Acknowledges messages by their opaque msg_ids.
* 
* Acked messages are permanently removed from the queue and will never be re-delivered.
*/
        public ack(
            msgQueueAckIn: MsgQueueAckIn,
            ): Promise<MsgQueueAckOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/queue/ack");

            request.setBody(
                    MsgQueueAckInSerializer._toJsonObject(
                        msgQueueAckIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    MsgQueueAckOutSerializer._fromJsonObject,
                );
            }

        

    }

