import { expect, test } from "vitest";

import * as STS from "../src/services/sts.js";
import * as DynamoDB from "../src/services/dynamodb.js";

import { getProfileCredentials } from "./getProfileCredentials";

const testCredentials = getProfileCredentials("test");

test("STS.GetCallerIdentity", async () => {
  const config = {
    region: "us-east-1",
    async credentials() {
      return testCredentials;
    },
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
    },
  };
  const response = await DynamoDB.ListTables(config, {
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
    },
  };
  for await (const response of DynamoDB.paginateListTables(config, {})) {
    console.log(response);
    expect(response).toMatchObject({
      TableNames: expect.arrayContaining([
        expect.stringMatching(/^(dev|test)-\w+$/),
      ]),
    });
  }
});
