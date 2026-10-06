// this file is @generated

import {
    type ClusterForceElectionIn,
    ClusterForceElectionInSerializer,
} from '../models/clusterForceElectionIn';
import {
    type ClusterForceElectionOut,
    ClusterForceElectionOutSerializer,
} from '../models/clusterForceElectionOut';
import {
    type ClusterForceNodeDowngradeIn,
    ClusterForceNodeDowngradeInSerializer,
} from '../models/clusterForceNodeDowngradeIn';
import {
    type ClusterForceNodeDowngradeOut,
    ClusterForceNodeDowngradeOutSerializer,
} from '../models/clusterForceNodeDowngradeOut';
import {
    type ClusterForceNodeUpgradeIn,
    ClusterForceNodeUpgradeInSerializer,
} from '../models/clusterForceNodeUpgradeIn';
import {
    type ClusterForceNodeUpgradeOut,
    ClusterForceNodeUpgradeOutSerializer,
} from '../models/clusterForceNodeUpgradeOut';
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
    }/** Force the cluster to conduct an election immediately */
    public forceElection(
        clusterForceElectionIn: ClusterForceElectionIn,
    ): Promise<ClusterForceElectionOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cluster-admin.force-election");

        request.setBody(
            ClusterForceElectionInSerializer._toJsonObject(clusterForceElectionIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterForceElectionOutSerializer._fromJsonObject,
        );
    }/**
* Request that the cluster upgrade the given node from "learner" to "voter"
* 
* This should only be invoked if a partition occurs during a learner process
* and you don't want to re-bootstrap the affected node.
*/
    public forceNodeUpgrade(
        clusterForceNodeUpgradeIn: ClusterForceNodeUpgradeIn,
    ): Promise<ClusterForceNodeUpgradeOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cluster-admin.force-node-upgrade");

        request.setBody(
            ClusterForceNodeUpgradeInSerializer._toJsonObject(clusterForceNodeUpgradeIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterForceNodeUpgradeOutSerializer._fromJsonObject,
        );
    }/**
* Request that the cluster upgrade the given node from "voter" to "learner"
* 
* This should only be invoked if you are testing the replication system
*/
    public forceNodeDowngrade(
        clusterForceNodeDowngradeIn: ClusterForceNodeDowngradeIn,
    ): Promise<ClusterForceNodeDowngradeOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cluster-admin.force-node-downgrade");

        request.setBody(
            ClusterForceNodeDowngradeInSerializer._toJsonObject(clusterForceNodeDowngradeIn)
        );
        
        return request.send(
            this.requestCtx,
            ClusterForceNodeDowngradeOutSerializer._fromJsonObject,
        );
    }
}

