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
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class ClusterAdmin {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Get information about the current cluster */
    public status(
    ): Promise<ClusterStatusOut> {
        const request = new DiomRequest(HttpMethod.GET, "/api/v1.cluster-admin.status");

        
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
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cluster-admin.initialize");

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
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cluster-admin.remove-node");

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
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cluster-admin.force-snapshot");

        request.setBody(
            ClusterForceSnapshotInSerializer._toJsonObject(clusterForceSnapshotIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterForceSnapshotOutSerializer._fromJsonObject,
        );
    }
}

