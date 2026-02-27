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
import { MsgsNamespace } from './msgsNamespace';
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export class Msgs {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new MsgsNamespace(this.requestCtx);
    }

    }

