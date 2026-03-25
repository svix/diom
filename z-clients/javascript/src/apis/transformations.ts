// this file is @generated

import {
    type TransformIn,
    TransformInSerializer,
} from '../models/transformIn';
import {
    type TransformOut,
    TransformOutSerializer,
} from '../models/transformOut';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class Transformations {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Execute a JavaScript transformation script against a payload and return the result. */
    public execute(
        transformIn: TransformIn,
    ): Promise<TransformOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.transformations.execute");

        request.setBody(
            TransformInSerializer._toJsonObject(transformIn)
        );
        
        return request.send(
            this.requestCtx,
            TransformOutSerializer._fromJsonObject,
        );
    }
}

