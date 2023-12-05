export interface ClientConfig {
    region: string;
    credentials: () => Promise<Credentials>;
}

export interface Credentials {
    accessKeyId: string;
    secretAccessKey: string;
    sessionToken?: string;
}

export interface Signer {
    (request: Request, clientConfig: ClientConfig): Promise<Request>;
}


export interface Operation {
    readonly input: unknown;
    readonly output: unknown;
}
