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
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Admin {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

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
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/admin/cluster/remove-node");

        request.setBody(
            ClusterRemoveNodeInSerializer._toJsonObject(clusterRemoveNodeIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterRemoveNodeOutSerializer._fromJsonObject,
        );
    }
}

