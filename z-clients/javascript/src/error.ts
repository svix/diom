export class DiomError extends Error {
  constructor(
    readonly operationId: string,
    message: string,
    options?: ErrorOptions,
  ) {
    super(message, options);
    this.name = new.target.name;
  }
}

export class ConnectionError extends DiomError {
  constructor(operationId: string, cause: unknown) {
    super(operationId, "connection error", { cause });
  }
}

export class InvalidInputError extends DiomError {
  constructor(
    operationId: string,
    readonly code: string,
    readonly detail: string,
    readonly location?: string,
  ) {
    super(operationId, errorResponseMessage(code, detail, location));
  }
}

export class OperationError extends DiomError {
  constructor(
    operationId: string,
    readonly code: string,
    readonly detail: string,
    readonly location?: string,
  ) {
    super(operationId, errorResponseMessage(code, detail, location));
  }
}

export class ServerError extends DiomError {
  constructor(
    operationId: string,
    readonly code: string,
    readonly detail: string,
    readonly location?: string,
  ) {
    super(operationId, errorResponseMessage(code, detail, location));
  }
}

export class OtherError extends DiomError {
  constructor(operationId: string, cause: unknown) {
    super(operationId, "internal error", { cause });
  }
}

function errorResponseMessage(code: string, detail: string, location?: string): string {
  let result = `code=${code}`;
  if (location !== undefined) {
    result += ` location="${location}"`;
  }
  const d = detail.replaceAll('"', '\\"');
  result += ` detail=${d}`;

  return result
}

export interface ErrorBody {
  type: string;
  code: string;
  detail: string;
  location?: string;
}

export function makeErrorFromResponse(operationId: string, respBody: ErrorBody): DiomError {
  switch (respBody.type) {
    case "invalid-input":
      return new InvalidInputError(operationId, respBody.code, respBody.detail, respBody.location);
    case "operation-error":
      return new OperationError(operationId, respBody.code, respBody.detail, respBody.location);
    case "server-error":
      return new ServerError(operationId, respBody.code, respBody.detail, respBody.location);
    default:
      return new OtherError(operationId, new RangeError(`invalid error type ${respBody.type}`));
  }
}
