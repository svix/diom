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
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class MsgsNamespace {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Creates or updates a msgs namespace with the given name. */
        public create(
            createNamespaceIn: CreateNamespaceIn,
            ): Promise<CreateNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/namespace/create");

            request.setBody(
                    CreateNamespaceInSerializer._toJsonObject(
                        createNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CreateNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    /** Gets a msgs namespace by name. */
        public get(
            getNamespaceIn: GetNamespaceIn,
            ): Promise<GetNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/namespace/get");

            request.setBody(
                    GetNamespaceInSerializer._toJsonObject(
                        getNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    GetNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    }

