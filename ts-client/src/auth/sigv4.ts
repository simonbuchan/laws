import type {ClientConfig} from "../service.js";

export interface ServiceConfig {
    name: string;
}

export function createSigner(serviceConfig: ServiceConfig) {
    return async (request: Request, clientConfig: ClientConfig): Promise<Request> => {
        return authenticate(request, clientConfig, serviceConfig);
    };
}

export async function authenticate(
    request: Request,
    clientConfig: ClientConfig,
    serviceConfig: ServiceConfig,
): Promise<Request> {
    const body = request.method === "GET" ? new ArrayBuffer(0) : await request.arrayBuffer();
    const payloadHash = await sha256Hex(body);

    const {accessKeyId, secretAccessKey, sessionToken} = await clientConfig.credentials();

    const url = new URL(request.url);
    const headers = new Headers(request.headers);
    headers.set('host', url.host);
    const date = headers.get('x-amz-date') ?? new Date().toISOString().replace(/[:\-]|\.\d{3}/g, '');
    headers.set('x-amz-date', date);
    headers.set('x-amz-content-sha256', payloadHash);
    if (sessionToken) {
        headers.set('x-amz-security-token', sessionToken);
    }

    const credentialScope = `${date.substring(0, 8)}/${clientConfig.region}/${serviceConfig.name}/aws4_request`;

    const signedHeaders = Array.from(headers.keys(), (key) => key.toLowerCase())
        .sort()
        .join(';');
    const canonicalHeaders = Array.from(headers.entries(), ([key, value]) => [key.toLowerCase(), value.trim()])
        .sort(([key1], [key2]) => key1.localeCompare(key2))
        .map(([key, value]) => `${key}:${value}\n`)
        .join('');
    const canonicalRequest = [
        request.method,
        url.pathname,
        url.search,
        canonicalHeaders,
        signedHeaders,
        payloadHash,
    ].join('\n');
    const canonicalRequestHash = await sha256Hex(toUtf8(canonicalRequest));
    const stringToSign = `AWS4-HMAC-SHA256\n${date}\n${credentialScope}\n${canonicalRequestHash}`;
    const kDate = await hmac(toUtf8(`AWS4${secretAccessKey}`), date.substring(0, 8));
    const kRegion = await hmac(kDate, clientConfig.region);
    const kService = await hmac(kRegion, serviceConfig.name);
    const kSigning = await hmac(kService, 'aws4_request');
    const signature = toHex(await hmac(kSigning, stringToSign));

    headers.set('Authorization', `AWS4-HMAC-SHA256 Credential=${accessKeyId}/${credentialScope},SignedHeaders=${signedHeaders},Signature=${signature}`);

    return new Request(request, { headers, body: request.method === "GET" ? undefined : body });
}

async function hmac(key: ArrayBuffer, value: string): Promise<ArrayBuffer> {
    const cryptoKey = await crypto
        .subtle
        .importKey('raw', key, {name: 'HMAC', hash: "sha-256"}, false, ['sign']);

    return await crypto.subtle.sign('HMAC', cryptoKey, toUtf8(value));
}

async function sha256Hex(payload: ArrayBuffer): Promise<string> {
    const arrayBuffer = await crypto
        .subtle
        .digest('SHA-256', payload);
    return toHex(arrayBuffer);
}

const utf8Encoder = new TextEncoder();

function toUtf8(str: string): Uint8Array {
    return utf8Encoder.encode(str);
}

function toHex(arrayBuffer: ArrayBuffer): string {
    return Array.from(new Uint8Array(arrayBuffer))
        .map((byte) => byte.toString(16).padStart(2, '0'))
        .join('');
}
