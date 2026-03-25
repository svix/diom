// this file is @generated
import type { DiomOptions } from "./options";
import { makeRequestContext, type DiomRequestContext } from "./request";
import { Admin } from "./apis/admin";
import { AuthToken } from "./apis/authToken";
import { Cache } from "./apis/cache";
import { Health } from "./apis/health";
import { Idempotency } from "./apis/idempotency";
import { Kv } from "./apis/kv";
import { Msgs } from "./apis/msgs";
import { RateLimit } from "./apis/rateLimit";
import { Transformations } from "./apis/transformations";

export {
  Admin,
  AuthToken,
  Cache,
  Health,
  Idempotency,
  Kv,
  Msgs,
  RateLimit,
  Transformations,
};

export class Diom {
  private readonly requestCtx: DiomRequestContext;

  public constructor(token: string, options: DiomOptions = {}) {
    this.requestCtx = makeRequestContext(token, options);
  }

  public get admin(){
    return new Admin(this.requestCtx);
  }

  public get authToken(){
    return new AuthToken(this.requestCtx);
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

  public get transformations(){
    return new Transformations(this.requestCtx);
  }
}