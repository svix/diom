// this file is @generated

import {
    type ClusterRemoveNodeIn,
    ClusterRemoveNodeInSerializer,
} from '../models/clusterRemoveNodeIn';
import {
    type ClusterRemoveNodeOut,
    ClusterRemoveNodeOutSerializer,
} from '../models/clusterRemoveNodeOut';
import { AdminCluster } from './adminCluster';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class Admin {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    public get cluster() {
        return new AdminCluster(this.requestCtx);
    }

    /**
* Remove a node from the cluster.
* 
* This operation executes immediately and the node must be wiped and reset
* before it can safely be added to the cluster.
*/
    public clusterRemoveNode(
        clusterRemoveNodeIn: ClusterRemoveNodeIn,
    ): Promise<ClusterRemoveNodeOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/admin/cluster/remove-node");

        request.setBody(
            ClusterRemoveNodeInSerializer._toJsonObject(clusterRemoveNodeIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterRemoveNodeOutSerializer._fromJsonObject,
        );
    }
}

