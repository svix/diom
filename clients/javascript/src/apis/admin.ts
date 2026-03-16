// this file is @generated

import {
    type ClusterStatusOut,
    ClusterStatusOutSerializer,
} from '../models/clusterStatusOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Admin {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Get information about the current cluster */
    public clusterStatus(
        ): Promise<ClusterStatusOut> {
        const request = new DiomRequest(HttpMethod.GET, "/api/v1/admin/cluster-status");

        
        return request.send(
            this.requestCtx,
            ClusterStatusOutSerializer._fromJsonObject,
        );
    }
}

