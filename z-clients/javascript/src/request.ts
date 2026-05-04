import { Packr } from "msgpackr";
import type { XOR } from "./util";
import { ConnectionError, type DiomError, type ErrorBody, makeErrorFromResponse, OtherError } from "./error";
import type { DiomOptions } from "./options";

export const LIB_VERSION = "0.2.3";
const USER_AGENT = `svix-libs/${LIB_VERSION}/javascript`;

/**
 * Matches Diom's `rmp_serde` map encoding (not msgpackr's record extensions).
 *
 * `skipValues: [undefined]` is required so that optional fields (e.g. `retention.bytes`)
 * are encoded as "missing key" rather than as MessagePack `nil` / extension values.
 */
const MSGPACK_CODEC = new Packr({
  useRecords: false,
  encodeUndefinedAsNil: true,
  skipValues: [undefined],
  // biome-ignore lint/suspicious/noExplicitAny: msgpackr type definitions missing `skipValues`
} as any);

const APPLICATION_MSGPACK = "application/msgpack";

export enum HttpMethod {
  GET = "GET",
  HEAD = "HEAD",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
  CONNECT = "CONNECT",
  OPTIONS = "OPTIONS",
  TRACE = "TRACE",
  PATCH = "PATCH",
}

export type DiomRequestContext = {
  /** The API base URL, like "https://api.svix.com" */
  baseUrl: string;
  /** The 'bearer' scheme access token */
  token: string;
  /** Time in milliseconds to wait for requests to get a response. */
  timeout?: number;
  /**
   * Custom fetch implementation to use for HTTP requests.
   * Useful for testing, adding custom middleware, or running in non-standard environments.
   */
  fetch?: typeof fetch;
} & XOR<
  {
    /** List of delays (in milliseconds) to wait before each retry attempt.*/
    retryScheduleInMs?: number[];
  },
  {
    /** The number of times the client will retry if a server-side error
     *  or timeout is received.
     *  Default: 2
     */
    numRetries?: number;
  }
>;

export function makeRequestContext(token: string, options: DiomOptions) {
  const baseUrl = options.serverUrl ?? "http://localhost:8624";

  if (options.retryScheduleInMs) {
    return {
      baseUrl,
      token,
      timeout: options.requestTimeout,
      retryScheduleInMs: options.retryScheduleInMs,
      fetch: options.fetch,
    };
  }
  if (options.numRetries) {
    return {
      baseUrl,
      token,
      timeout: options.requestTimeout,
      numRetries: options.numRetries,
      fetch: options.fetch,
    };
  }
  return {
    baseUrl,
    token,
    timeout: options.requestTimeout,
    fetch: options.fetch,
  };
}

type QueryParameter = string | boolean | number | Date | string[] | null | undefined;

export class DiomRequest {
  constructor(
    private readonly method: HttpMethod,
    private path: string
  ) { }

  private body?: BodyInit;
  private queryParams: Record<string, string> = {};
  private headerParams: Record<string, string> = {};

  public setPathParam(name: string, value: string) {
    const newPath = this.path.replace(`{${name}}`, encodeURIComponent(value));
    if (this.path === newPath) {
      throw new Error(`path parameter ${name} not found`);
    }
    this.path = newPath;
  }

  public setQueryParams(params: { [name: string]: QueryParameter }) {
    for (const [name, value] of Object.entries(params)) {
      this.setQueryParam(name, value);
    }
  }

  public setQueryParam(name: string, value: QueryParameter) {
    if (value === undefined || value === null) {
      return;
    }

    if (typeof value === "string") {
      this.queryParams[name] = value;
    } else if (typeof value === "boolean" || typeof value === "number") {
      this.queryParams[name] = value.toString();
    } else if (value instanceof Date) {
      this.queryParams[name] = value.toISOString();
    } else if (Array.isArray(value)) {
      if (value.length > 0) {
        this.queryParams[name] = value.join(",");
      }
    } else {
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      const _assert_unreachable: never = value;
      throw new Error(`query parameter ${name} has unsupported type`);
    }
  }

  public setHeaderParam(name: string, value?: string) {
    if (value === undefined) {
      return;
    }

    this.headerParams[name] = value;
  }

  // biome-ignore lint/suspicious/noExplicitAny: intentional any
  public setBody(value: any) {
    this.body = MSGPACK_CODEC.pack(value) as BodyInit;
  }

  /**
   * Send this request, returning the request body as a caller-specified type.
   *
   * If the server returns a 422 error, an `ApiException<HTTPValidationError>` is thrown.
   * If the server returns another 4xx error, an `ApiException<HttpErrorOut>` is thrown.
   *
   * If the server returns a 5xx error, the request is retried up to two times with exponential backoff.
   * If retries are exhausted, an `ApiException<HttpErrorOut>` is thrown.
   */
  public async send<R>(
    ctx: DiomRequestContext,
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    parseResponseBody: (decoded: any) => R
  ): Promise<R> {
    const operationId = this.operationId();
    const response = await this.sendInner(ctx, operationId);
    if (response.status === 204) {
      return <R>null;
    }
    const decoded = await readMsgpackResponse(response, operationId);
    return parseResponseBody(decoded);
  }

  /** Same as `send`, but the response body is discarded, not parsed. */
  public async sendNoResponseBody(ctx: DiomRequestContext): Promise<void> {
    await this.sendInner(ctx, this.operationId());
  }

  private operationId(): string {
    if (this.path.startsWith("/api/")) {
      return this.path.substring("/api/".length);
    } else {
      console.error("request path must begin with /api/");
      return "[ERROR]";
    }
  }

  private async sendInner(ctx: DiomRequestContext, operationId: string): Promise<Response> {
    const url = new URL(ctx.baseUrl + this.path);
    for (const [name, value] of Object.entries(this.queryParams)) {
      url.searchParams.set(name, value);
    }

    const randomId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);

    if (this.body != null) {
      this.headerParams["content-type"] = APPLICATION_MSGPACK;
    }
    // Cloudflare Workers fail if the credentials option is used in a fetch call.
    // This work around that. Source:
    // https://github.com/cloudflare/workers-sdk/issues/2514#issuecomment-21.85.0014
    const isCredentialsSupported = "credentials" in Request.prototype;

    let response: Response;
    try {
      response = await sendWithRetry(
        url,
        {
          method: this.method.toString(),
          body: this.body,
          headers: {
            accept: APPLICATION_MSGPACK,
            authorization: `Bearer ${ctx.token}`,
            "user-agent": USER_AGENT,
            "diom-req-id": randomId.toString(),
            ...this.headerParams,
          },
          credentials: isCredentialsSupported ? "same-origin" : undefined,
          signal: ctx.timeout !== undefined ? AbortSignal.timeout(ctx.timeout) : undefined,
        },
        ctx.retryScheduleInMs,
        ctx.retryScheduleInMs?.[0],
        ctx.retryScheduleInMs?.length || ctx.numRetries,
        ctx.fetch
      );
    } catch(err) {
      throw new ConnectionError(operationId, err);
    }
    return filterResponseForErrors(operationId, response);
  }
}

async function filterResponseForErrors(operationId: string, response: Response): Promise<Response> {
  if (response.status < 300) {
    return response;
  }

  const decoded = await readMsgpackResponse(response, operationId);

  let error: DiomError;
  try {
    error = makeErrorFromResponse(operationId, decoded as ErrorBody);
  } catch(err) {
    // if an error happens during field access on the resulting object, throw 'OtherError'
    throw new OtherError(operationId, err);
  }

  throw error;
}

async function readMsgpackResponse(response: Response, operationId: string): Promise<unknown> {
  let raw: Uint8Array;
  try {
    raw = new Uint8Array(await response.arrayBuffer());
  } catch (err) {
    throw new ConnectionError(operationId, err);
  }

  try {
    return decodeMsgpackBody(raw);
  } catch(err) {
    throw new OtherError(operationId, err);
  }
}

function decodeMsgpackBody(raw: Uint8Array): unknown {
  if (raw.byteLength === 0) {
    return null;
  }
  return MSGPACK_CODEC.unpack(raw);
}

type SvixRequestInit = RequestInit & {
  headers: Record<string, string>;
};

async function sendWithRetry(
  url: URL,
  init: SvixRequestInit,
  retryScheduleInMs?: number[],
  // FIXME: Are the defaults here ever used? It seems like only retryCount is.
  nextInterval = 50,
  triesLeft = 0,
  fetchImpl: typeof fetch = fetch,
  retryCount = 1
): Promise<Response> {
  const sleep = (interval: number) =>
    new Promise((resolve) => setTimeout(resolve, interval));

  try {
    const response = await fetchImpl(url, init);
    if (triesLeft <= 0 || response.status < 500) {
      return response;
    }
  } catch (e) {
    if (triesLeft <= 0) {
      throw e;
    }
  }

  await sleep(nextInterval);
  init.headers["diom-retry-count"] = retryCount.toString();
  nextInterval = retryScheduleInMs?.[retryCount] || nextInterval * 2;
  return await sendWithRetry(
    url,
    init,
    retryScheduleInMs,
    nextInterval,
    --triesLeft,
    fetchImpl,
    ++retryCount
  );
}
