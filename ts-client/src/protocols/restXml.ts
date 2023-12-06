import * as xml from "../xml.js";

export interface ServiceConfig {
  readonly xmlNamespace: string;
  readonly xmlNamespacePrefix?: string;
}

export interface OperationConfig {
  readonly input: string;
  readonly output: string;
  readonly method: string;
  readonly path: string;
}

export function inputRequest(
  service: ServiceConfig,
  endpoint: string,
  operation: OperationConfig,
  input: unknown,
): Request {
  return new Request(`https://${endpoint}${operation.path}`, {
    method: operation.method,
    headers: {
      "Content-Type": "application/xml",
    },
    body: xml.formatRequest(
      input,
      operation.input,
      service.xmlNamespace,
      service.xmlNamespacePrefix,
    ),
  });
}

export async function outputResult(
  _service: ServiceConfig,
  operation: OperationConfig,
  response: Response,
): Promise<unknown> {
  return xml.parseResponse(await response.text(), operation.output);
}
