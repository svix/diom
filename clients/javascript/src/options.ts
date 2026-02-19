import type { XOR } from "./util";

export type CoyoteOptions = {
  serverUrl?: string;
  debug?: boolean;
  /** Time in milliseconds to wait for requests to get a response. */
  requestTimeout?: number;
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
