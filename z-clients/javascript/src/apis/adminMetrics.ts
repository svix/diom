// this file is @generated

import {
    type GetMetricsOut,
    GetMetricsOutSerializer,
} from '../models/getMetricsOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class AdminMetrics {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver) */
    public get(
    ): Promise<GetMetricsOut> {
        const request = new DiomRequest(HttpMethod.GET, "/api/v1.admin.metrics.get");

        
        return request.send(
            this.requestCtx,
            GetMetricsOutSerializer._fromJsonObject,
        );
    }
}

