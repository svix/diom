// this file is @generated

import {
    type ClusterForceSnapshotIn,
    ClusterForceSnapshotInSerializer,
} from '../models/clusterForceSnapshotIn';
import {
    type ClusterForceSnapshotOut,
    ClusterForceSnapshotOutSerializer,
} from '../models/clusterForceSnapshotOut';
import {
    type ClusterInitializeIn,
    ClusterInitializeInSerializer,
} from '../models/clusterInitializeIn';
import {
    type ClusterInitializeOut,
    ClusterInitializeOutSerializer,
} from '../models/clusterInitializeOut';
import {
    type ClusterRemoveNodeIn,
    ClusterRemoveNodeInSerializer,
} from '../models/clusterRemoveNodeIn';
import {
    type ClusterRemoveNodeOut,
    ClusterRemoveNodeOutSerializer,
} from '../models/clusterRemoveNodeOut';
import {
    type ClusterStatusOut,
    ClusterStatusOutSerializer,
} from '../models/clusterStatusOut';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class AdminCluster {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Get information about the current cluster */
    public status(
    ): Promise<ClusterStatusOut> {
        const request = new CoyoteRequest(HttpMethod.GET, "/api/v1.admin.cluster.status");

        
        return request.send(
            this.requestCtx,
            ClusterStatusOutSerializer._fromJsonObject,
        );
    }/**
* Initialize this node as the leader of a new cluster
* 
* This operation may only be performed against a node which has not been
* initialized and is not currently a member of a cluster.
*/
    public initialize(
        clusterInitializeIn: ClusterInitializeIn,
    ): Promise<ClusterInitializeOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.admin.cluster.initialize");

        request.setBody(
            ClusterInitializeInSerializer._toJsonObject(clusterInitializeIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterInitializeOutSerializer._fromJsonObject,
        );
    }/**
* Remove a node from the cluster.
* 
* This operation executes immediately and the node must be wiped and reset
* before it can safely be added to the cluster.
*/
    public removeNode(
        clusterRemoveNodeIn: ClusterRemoveNodeIn,
    ): Promise<ClusterRemoveNodeOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.admin.cluster.remove-node");

        request.setBody(
            ClusterRemoveNodeInSerializer._toJsonObject(clusterRemoveNodeIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterRemoveNodeOutSerializer._fromJsonObject,
        );
    }/** Force the cluster to take a snapshot immediately */
    public forceSnapshot(
        clusterForceSnapshotIn: ClusterForceSnapshotIn,
    ): Promise<ClusterForceSnapshotOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.admin.cluster.force-snapshot");

        request.setBody(
            ClusterForceSnapshotInSerializer._toJsonObject(clusterForceSnapshotIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterForceSnapshotOutSerializer._fromJsonObject,
        );
    }
}

