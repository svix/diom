// this file is @generated

import {
    type Ack,
    AckSerializer,
} from '../models/ack';
import {
    type AckMsgRangeIn,
    AckMsgRangeInSerializer,
} from '../models/ackMsgRangeIn';
import {
    type AckMsgRangeOut,
    AckMsgRangeOutSerializer,
} from '../models/ackMsgRangeOut';
import {
    type AckOut,
    AckOutSerializer,
} from '../models/ackOut';
import {
    type AppendToStreamIn,
    AppendToStreamInSerializer,
} from '../models/appendToStreamIn';
import {
    type AppendToStreamOut,
    AppendToStreamOutSerializer,
} from '../models/appendToStreamOut';
import {
    type CreateStreamIn,
    CreateStreamInSerializer,
} from '../models/createStreamIn';
import {
    type CreateStreamOut,
    CreateStreamOutSerializer,
} from '../models/createStreamOut';
import {
    type DlqIn,
    DlqInSerializer,
} from '../models/dlqIn';
import {
    type DlqOut,
    DlqOutSerializer,
} from '../models/dlqOut';
import {
    type FetchFromStreamIn,
    FetchFromStreamInSerializer,
} from '../models/fetchFromStreamIn';
import {
    type FetchFromStreamOut,
    FetchFromStreamOutSerializer,
} from '../models/fetchFromStreamOut';
import {
    type RedriveIn,
    RedriveInSerializer,
} from '../models/redriveIn';
import {
    type RedriveOut,
    RedriveOutSerializer,
} from '../models/redriveOut';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export interface StreamCreateOptions {
        idempotencyKey?: string;
        }

    export interface StreamAppendOptions {
        idempotencyKey?: string;
        }

    export interface StreamFetchOptions {
        idempotencyKey?: string;
        }

    export interface StreamFetchLockingOptions {
        idempotencyKey?: string;
        }

    export interface StreamAckRangeOptions {
        idempotencyKey?: string;
        }

    export interface StreamAckOptions {
        idempotencyKey?: string;
        }

    export interface StreamDlqOptions {
        idempotencyKey?: string;
        }

    export interface StreamRedriveOptions {
        idempotencyKey?: string;
        }

    export class Stream {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Upserts a new Stream with the given name. */
        public create(
            createStreamIn: CreateStreamIn,
            options?: StreamCreateOptions,
            ): Promise<CreateStreamOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/create");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    CreateStreamInSerializer._toJsonObject(
                        createStreamIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CreateStreamOutSerializer._fromJsonObject,
                );
            }

        

    /** Appends messages to the stream. */
        public append(
            appendToStreamIn: AppendToStreamIn,
            options?: StreamAppendOptions,
            ): Promise<AppendToStreamOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/append");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    AppendToStreamInSerializer._toJsonObject(
                        appendToStreamIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    AppendToStreamOutSerializer._fromJsonObject,
                );
            }

        

    /**
* Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
* 
* Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
* messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
* until the visibility timeout expires, or the messages are acked.
*/
        public fetch(
            fetchFromStreamIn: FetchFromStreamIn,
            options?: StreamFetchOptions,
            ): Promise<FetchFromStreamOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/fetch");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    FetchFromStreamInSerializer._toJsonObject(
                        fetchFromStreamIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    FetchFromStreamOutSerializer._fromJsonObject,
                );
            }

        

    /**
* Fetches messages from the stream, locking over the consumer group.
* 
* This call prevents other consumers within the same consumer group from reading from the stream
* until either the visibility timeout expires, or the last message in the batch is acknowledged.
*/
        public fetchLocking(
            fetchFromStreamIn: FetchFromStreamIn,
            options?: StreamFetchLockingOptions,
            ): Promise<FetchFromStreamOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/fetch-locking");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    FetchFromStreamInSerializer._toJsonObject(
                        fetchFromStreamIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    FetchFromStreamOutSerializer._fromJsonObject,
                );
            }

        

    /** Acks the messages for the consumer group, allowing more messages to be consumed. */
        public ackRange(
            ackMsgRangeIn: AckMsgRangeIn,
            options?: StreamAckRangeOptions,
            ): Promise<AckMsgRangeOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/ack-range");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    AckMsgRangeInSerializer._toJsonObject(
                        ackMsgRangeIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    AckMsgRangeOutSerializer._fromJsonObject,
                );
            }

        

    /** Acks a single message. */
        public ack(
            ack: Ack,
            options?: StreamAckOptions,
            ): Promise<AckOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/ack");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    AckSerializer._toJsonObject(
                        ack,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    AckOutSerializer._fromJsonObject,
                );
            }

        

    /** Moves a message to the dead letter queue. */
        public dlq(
            dlqIn: DlqIn,
            options?: StreamDlqOptions,
            ): Promise<DlqOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/dlq");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    DlqInSerializer._toJsonObject(
                        dlqIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    DlqOutSerializer._fromJsonObject,
                );
            }

        

    /** Redrives messages from the dead letter queue back to the stream. */
        public redrive(
            redriveIn: RedriveIn,
            options?: StreamRedriveOptions,
            ): Promise<RedriveOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/stream/redrive-dlq");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    RedriveInSerializer._toJsonObject(
                        redriveIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    RedriveOutSerializer._fromJsonObject,
                );
            }

        

    }

