// this file is @generated
import type { DiomOptions } from "./options";
import { makeRequestContext, type DiomRequestContext } from "./request";
import { Cache } from "./apis/cache";
import { Health } from "./apis/health";
import { Idempotency } from "./apis/idempotency";
import { Kv } from "./apis/kv";
import { Msgs } from "./apis/msgs";
import { RateLimiter } from "./apis/rateLimiter";
import { Stream } from "./apis/stream";

export {
  Cache,
  Health,
  Idempotency,
  Kv,
  Msgs,
  RateLimiter,
  Stream,
};

export class Diom {
  private readonly requestCtx: DiomRequestContext;

  public constructor(token: string, options: DiomOptions = {}) {
    this.requestCtx = makeRequestContext(token, options);
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

  public get rateLimiter(){
    return new RateLimiter(this.requestCtx);
  }

  public get stream(){
    return new Stream(this.requestCtx);
  }
}