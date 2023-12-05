import * as fs from "node:fs";

import {expect, test} from "vitest";

import * as STS from "../src/services/sts";
import * as DynamoDB from "../src/services/dynamodb";

function getProfileCredentials(profile: string) {
    const source = fs.readFileSync(`${process.env.HOME}/.aws/credentials`, "utf8");
    const config = parseIni(source);
    const profileConfig = config[profile];
    if (!profileConfig) {
        throw new Error(`Profile not found: ${profile}`);
    }
    return {
        accessKeyId: profileConfig.aws_access_key_id,
        secretAccessKey: profileConfig.aws_secret_access_key,
        sessionToken: profileConfig.aws_session_token,
    };
}

function parseIni(ini: string) {
    const config: Record<string, Record<string, string>> = {};
    let section = "";
    for (const line of ini.split(/\r?\n/g)) {
        if (!line || line.match(/^\s*[;#]/)) {
            continue;
        }
        const sectionMatch = line.match(/^\s*\[(.*)]\s*$/);
        const valueMatch = line.match(/^\s*(.*?)\s*=\s*(.*?)\s*$/);
        if (sectionMatch) {
            section = sectionMatch[1];
        } else if (valueMatch) {
            config[section] ??= {};
            config[section][valueMatch[1]] = valueMatch[2];
        } else {
            throw new Error(`Invalid line: ${JSON.stringify(line)}`);
        }
    }
    return config;
}

const testCredentials = getProfileCredentials("test");

test("STS.GetCallerIdentity", async () => {
    const config = {
        region: "us-east-1",
        async credentials() {
            return testCredentials;
        }
    };
    const response = await STS.GetCallerIdentity(config, {});
    expect(response).toMatchObject({
        Account: expect.stringMatching(/^\d{12}$/),
        Arn: expect.stringMatching(/^arn:aws:iam::\d{12}:user\/\w+$/),
        UserId: expect.stringMatching(/^AIDA\w{17}$/),
    });
});

test("DynamoDB.ListTables", async () => {
    const config = {
        region: "us-west-2",
        async credentials() {
            return testCredentials;
        }
    };
    const response = await DynamoDB.ListTables(config,{
        Limit: 10,
    });
    console.log(response);
    expect(response).toMatchObject({
        TableNames: expect.arrayContaining([
            expect.stringMatching(/^(dev|test)-\w+$/),
        ]),
    });
});

test("DynamoDB.paginateListTables", async () => {
    const config = {
        region: "us-west-2",
        async credentials() {
            return testCredentials;
        }
    };
    for await (const response of DynamoDB.paginateListTables(config,{})) {
        console.log(response);
        expect(response).toMatchObject({
            TableNames: expect.arrayContaining([
                expect.stringMatching(/^(dev|test)-\w+$/),
            ]),
        });
    }
});
