export interface ServiceConfig {
}

export interface OperationConfig {
    readonly method: string;
    readonly path: string;
}

export function inputRequest(
    _service: ServiceConfig,
    endpoint: string,
    operation: OperationConfig,
    input: unknown,
): Request {
    return new Request(`https://${endpoint}${operation.path}`, {
        method: operation.method,
        headers: {
            "Content-Type": "application/json",
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
