export interface ServiceConfig {
    readonly targetPrefix: string;
}

export type OperationConfig = string;

export function inputRequest(
    service: ServiceConfig,
    endpoint: string,
    operation: OperationConfig,
    input: unknown,
): Request {
    return new Request(`https://${endpoint}/`, {
        method: "POST",
        headers: {
            "Content-Type": "application/x-amz-json-1.0",
            "X-Amz-Target": `${service.targetPrefix}.${operation}`,
        },
        body: JSON.stringify(input),
    });
}

export async function outputResult(
    _service: ServiceConfig,
    _operation: OperationConfig,
    response: Response,
): Promise<unknown> {
    return await response.json();
}
