// this file is @generated

import {
    type CreateNamespaceIn,
    CreateNamespaceInSerializer,
} from '../models/createNamespaceIn';
import {
    type CreateNamespaceOut,
    CreateNamespaceOutSerializer,
} from '../models/createNamespaceOut';
import {
    type GetNamespaceIn,
    GetNamespaceInSerializer,
} from '../models/getNamespaceIn';
import {
    type GetNamespaceOut,
    GetNamespaceOutSerializer,
} from '../models/getNamespaceOut';
import {
    type PublishIn,
    PublishInSerializer,
} from '../models/publishIn';
import {
    type PublishOut,
    PublishOutSerializer,
} from '../models/publishOut';
import { MsgsNamespace } from './msgsNamespace';
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export class Msgs {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new MsgsNamespace(this.requestCtx);
    }

    /** Publishes messages to a topic within a namespace. */
        public publish(
            publishIn: PublishIn,
            ): Promise<PublishOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/publish");

            request.setBody(
                    PublishInSerializer._toJsonObject(
                        publishIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    PublishOutSerializer._fromJsonObject,
                );
            }

        

    }

