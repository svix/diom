// this file is @generated
import type { DiomOptions } from "./options";
import { makeRequestContext, type DiomRequestContext } from "./request";
import { Admin } from "./apis/admin";
import { Cache } from "./apis/cache";
import { ClusterAdmin } from "./apis/clusterAdmin";
import { Health } from "./apis/health";
import { Idempotency } from "./apis/idempotency";
import { Kv } from "./apis/kv";
import { Msgs } from "./apis/msgs";
import { RateLimit } from "./apis/rateLimit";

export {
  Admin,
  Cache,
  ClusterAdmin,
  Health,
  Idempotency,
  Kv,
  Msgs,
  RateLimit,
};

export class Diom {
  private readonly requestCtx: DiomRequestContext;

  public constructor(token: string, options: DiomOptions = {}) {
    this.requestCtx = makeRequestContext(token, options);
  }

  public get admin(){
    return new Admin(this.requestCtx);
  }

  public get cache(){
    return new Cache(this.requestCtx);
  }

  public get clusterAdmin(){
    return new ClusterAdmin(this.requestCtx);
  }

  public get health(){
    return new Health(this.requestCtx);
  }

  public get idempotency(){
    return new Idempotency(this.requestCtx);
  }

  public get kv(){
    return new Kv(this.requestCtx);
  }

  public get msgs(){
    return new Msgs(this.requestCtx);
  }

  public get rateLimit(){
    return new RateLimit(this.requestCtx);
  }
}