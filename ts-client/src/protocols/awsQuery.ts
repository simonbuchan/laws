import * as xml from "../xml.js";

export interface ServiceConfig {
  readonly version: string;
  readonly xmlNamespace: string;
  readonly xmlNamespacePrefix?: string;
}

export interface OperationConfig {
  readonly action: string;
  readonly output: string;
}

export function inputRequest(
  service: ServiceConfig,
  endpoint: string,
  operation: OperationConfig,
  input: unknown,
): Request {
  return new Request(`https://${endpoint}/`, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    body: new URLSearchParams({
      ...(input as Record<string, string>),
      Action: operation.action,
      Version: service.version,
    }),
  });
}

export async function outputResult(
  service: ServiceConfig,
  operation: OperationConfig,
  response: Response,
): Promise<unknown> {
  return xml.parseResponse(
    await response.text(),
    operation.output,
    service.xmlNamespace,
    service.xmlNamespacePrefix,
  );
}
