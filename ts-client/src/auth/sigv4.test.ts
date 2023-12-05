// example from https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html

import { expect, test } from "vitest";
import {authenticate} from "./sigv4.js";

test("sigv4", async () => {
    const credentials =  {
        accessKeyId: "AKIAIOSFODNN7EXAMPLE",
        secretAccessKey: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
    };

    const request = new Request("https://examplebucket.s3.amazonaws.com/test.txt", {
        method: "GET",
        headers: {
            Range: "bytes=0-9",
            // x-amz-content-sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "x-amz-date": "20130524T000000Z",
        },
    });

    const clientConfig = {
        region: "us-east-1",
        credentials: async () => credentials,
    };

    const serviceConfig = {
        name: "s3",
    };

    const signedRequest = await authenticate(request, clientConfig, serviceConfig);
    expect(signedRequest.headers.get("Authorization")).toBe(
        "AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request,SignedHeaders=host;range;x-amz-content-sha256;x-amz-date,Signature=f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41"
    );
});
