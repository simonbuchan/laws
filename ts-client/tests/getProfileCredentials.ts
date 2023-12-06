import * as fs from "node:fs";

export function getProfileCredentials(profile: string) {
  const source = fs.readFileSync(
    `${process.env.HOME}/.aws/credentials`,
    "utf8",
  );
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
