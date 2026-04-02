// this file is @generated
import type { CoyoteOptions } from "./options";
import { makeRequestContext, type CoyoteRequestContext } from "./request";
import { Admin } from "./apis/admin";
import { Cache } from "./apis/cache";
import { Health } from "./apis/health";
import { Idempotency } from "./apis/idempotency";
import { Kv } from "./apis/kv";
import { Msgs } from "./apis/msgs";
import { RateLimit } from "./apis/rateLimit";

export {
  Admin,
  Cache,
  Health,
  Idempotency,
  Kv,
  Msgs,
  RateLimit,
};

export class Coyote {
  private readonly requestCtx: CoyoteRequestContext;

  public constructor(token: string, options: CoyoteOptions = {}) {
    this.requestCtx = makeRequestContext(token, options);
  }

  public get admin(){
    return new Admin(this.requestCtx);
  }

  public get cache(){
    return new Cache(this.requestCtx);
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