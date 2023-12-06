use std::path::Path;

use miette::{IntoDiagnostic, Result};

use laws_schema as schema;

enum Protocol {
    AwsJson1_0,
    AwsJson1_1,
    AwsQuery,
    Ec2Query,
    RestJson1,
    RestXml,
}

pub fn write_service(model: &schema::Model, path: &Path) -> Result<()> {
    use std::fs;
    use std::io::Write;

    let service_model_name = path
        .file_stem()
        .ok_or(miette::diagnostic!("no file stem for service model"))?
        .to_str()
        .ok_or(miette::diagnostic!("file stem is not valid UTF-8"))?;

    let mut f = std::io::BufWriter::new(fs::File::create(path).into_diagnostic()?);

    // Write service. Only have one service per model at the moment, but we should be able to
    // write an API per version.
    let (service_name, service) = model
        .shapes
        .iter()
        .find_map(|(name, shape)| match shape {
            schema::Shape::Service(shape) => Some((name.name.clone(), shape)),
            _ => None,
        })
        .ok_or(miette::diagnostic!("no service found in model"))?;

    let protocol;
    if service.traits.protocols_aws_json_1_0.is_some() {
        protocol = Protocol::AwsJson1_0;
        writeln!(
            f,
            r#"import * as protocol from "../protocols/awsJson1_0.js";"#,
        )
        .into_diagnostic()?;
    } else if service.traits.protocols_aws_json_1_1.is_some() {
        protocol = Protocol::AwsJson1_1;
        writeln!(
            f,
            r#"import * as protocol from "../protocols/awsJson1_1.js";"#,
        )
        .into_diagnostic()?;
    } else if service.traits.protocols_aws_query.is_some() {
        protocol = Protocol::AwsQuery;
        writeln!(
            f,
            r#"import * as protocol from "../protocols/awsQuery.js";"#,
        )
        .into_diagnostic()?;
    } else if service.traits.protocols_ec2_query.is_some() {
        protocol = Protocol::Ec2Query;
        writeln!(
            f,
            r#"import * as protocol from "../protocols/ec2Query.js";"#,
        )
        .into_diagnostic()?;
    } else if service.traits.protocols_rest_json_1.is_some() {
        protocol = Protocol::RestJson1;
        writeln!(
            f,
            r#"import * as protocol from "../protocols/restJson1.js";"#,
        )
        .into_diagnostic()?;
    } else if service.traits.protocols_rest_xml.is_some() {
        protocol = Protocol::RestXml;
        writeln!(f, r#"import * as protocol from "../protocols/restXml.js";"#,)
            .into_diagnostic()?;
    } else {
        miette::bail!("unimplemented or missing protocol");
    }

    writeln!(f, r#"import type {{ ClientConfig }} from "../service.js";"#,).into_diagnostic()?;

    if service.traits.auth_sigv4.is_some() {
        writeln!(f, r#"import {{ authenticate }} from "../auth/sigv4.js";"#,).into_diagnostic()?;
    }
    writeln!(f).into_diagnostic()?;

    // Write service config
    writeln!(f, r#"const service: protocol.ServiceConfig = {{"#).into_diagnostic()?;
    match protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
            writeln!(f, r#"  targetPrefix: {service_name:?},"#).into_diagnostic()?;
        }
        Protocol::AwsQuery => {
            let namespace = service
                .traits
                .xml_namespace
                .as_ref()
                .ok_or(miette::diagnostic!(
                    "no smithy.api#xmlNamespace service trait with aws.protocols#awsQuery"
                ))?;
            writeln!(
                f,
                r#"  xmlNamespace: {namespace:?},"#,
                namespace = namespace.uri
            )
            .into_diagnostic()?;
            if let Some(prefix) = &namespace.prefix {
                writeln!(f, r#"  xmlNamespacePrefix: {prefix:?},"#).into_diagnostic()?;
            }
            writeln!(f, r#"  version: {version:?},"#, version = service.version)
                .into_diagnostic()?;
        }
        Protocol::RestXml => {
            let namespace = service
                .traits
                .xml_namespace
                .as_ref()
                .ok_or(miette::diagnostic!(
                    "no smithy.api#xmlNamespace service trait with aws.protocols#restXml"
                ))?;
            writeln!(
                f,
                r#"  xmlNamespace: {namespace:?},"#,
                namespace = namespace.uri
            )
            .into_diagnostic()?;
            if let Some(prefix) = &namespace.prefix {
                writeln!(f, r#"  xmlNamespacePrefix: {prefix:?},"#).into_diagnostic()?;
            }
        }
        _ => {}
    }
    match protocol {
        Protocol::AwsQuery | Protocol::RestXml => {}
        _ => {}
    }
    writeln!(f, "}};").into_diagnostic()?;
    writeln!(f).into_diagnostic()?;

    // Write send function
    doc_comment(&mut f, "", &service.traits.documentation)?;
    writeln!(
        f,
        r#"export async function send<const Name extends keyof OperationMap>(
    client: ClientConfig,
    operation: protocol.OperationConfig,
    input: OperationMap[Name]['input'],
): Promise<OperationMap[Name]['output']> {{"#
    )
    .into_diagnostic()?;

    let endpoint_prefix = &service
        .traits
        .service
        .endpoint_prefix
        .as_deref()
        .unwrap_or(service_model_name);

    writeln!(
        f,
        r#"  const endpoint = `{endpoint_prefix}.${{client.region}}.amazonaws.com`;
  let request = protocol.inputRequest(service, endpoint, operation, input);"#
    )
    .into_diagnostic()?;

    writeln!(f).into_diagnostic()?;

    if let Some(auth) = &service.traits.auth_sigv4 {
        writeln!(
            f,
            "  request = await authenticate(request, client, {{ name: {name:?} }});",
            name = auth.name
        )
        .into_diagnostic()?;
    }

    writeln!(
        f,
        r#"  const response = await fetch(request);
  if (!response.ok) {{
    const body = await response.text();
    throw new Error(`HTTP ${{response.status}} ${{response.statusText}}:\n${{body}}`);
  }}

  return await protocol.outputResult(service, operation, response) as OperationMap[Name]['output'];
}}
"#
    )
    .into_diagnostic()?;

    // Write operation input / output map.
    writeln!(f, "export interface OperationMap {{").into_diagnostic()?;
    for (name, shape) in &model.shapes {
        if let schema::Shape::Operation(shape) = shape {
            doc_comment(&mut f, "    ", &shape.traits.documentation)?;
            writeln!(
                f,
                "    readonly {name}: {{ readonly input: {input}; readonly output: {output} }};",
                name = name.name,
                input = shape_id_to_ts(&shape.input.target),
                output = shape_id_to_ts(&shape.output.target),
            )
            .into_diagnostic()?;
        }
    }
    writeln!(f, "}}").into_diagnostic()?;
    writeln!(f).into_diagnostic()?;

    // Write operations.
    for (name, shape) in &model.shapes {
        if let schema::Shape::Operation(shape) = shape {
            let name = &name.name;
            let input = shape_id_to_ts(&shape.input.target);
            let output = shape_id_to_ts(&shape.output.target);

            writeln!(
                f,
                r#"export async function {name}(client: ClientConfig, input: {input}): Promise<{output}> {{"#
            )
            .into_diagnostic()?;
            match protocol {
                Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
                    writeln!(f, "  const operation: protocol.OperationConfig = {name:?};")
                        .into_diagnostic()?;
                }
                Protocol::AwsQuery => {
                    writeln!(
                        f,
                        "  const operation: protocol.OperationConfig = {{ action: {name:?}, output: {output_name:?} }};",
                        output_name = shape_id_to_ts(&shape.output.target),
                    )
                    .into_diagnostic()?;
                }
                Protocol::RestJson1 => {
                    let http = shape.traits.http.as_ref().ok_or(miette::diagnostic!(
                        "no http trait for operation with protocols#restJson1"
                    ))?;

                    // expand "/foo/{bar}" => `/foo/${input.bar}`,
                    //    and "/foo/{bar+}/baz" => `/foo/${input.bar/baz`
                    let path = http.uri.replace('{', "${input.").replace("+}", "}");
                    writeln!(
                        f,
                        "  const operation: protocol.OperationConfig = {{ method: {method:?}, path: `{path}` }};",
                        method = http.method,
                    )
                        .into_diagnostic()?;
                }
                Protocol::RestXml => {
                    let http = shape.traits.http.as_ref().ok_or(miette::diagnostic!(
                        "no http trait for operation with protocols#restXml"
                    ))?;

                    // expand "/foo/{bar}" => `/foo/${input.bar}`,
                    //    and "/foo/{bar+}/baz" => `/foo/${input.bar/baz`
                    let path = http.uri.replace('{', "${input.").replace("+}", "}");
                    writeln!(
                        f,
                        "  const operation: protocol.OperationConfig = {{ input: {input_name:?}, output: {output_name:?}, method: {method:?}, path: `{path}` }};",
                        input_name = shape_id_to_ts(&shape.input.target),
                        output_name = shape_id_to_ts(&shape.output.target),
                        method = http.method,
                    )
                    .into_diagnostic()?;
                }
                _ => {
                    miette::bail!("unimplemented protocol");
                }
            }
            writeln!(
                f,
                "  return await send<{name:?}>(client, operation, input);"
            )
            .into_diagnostic()?;
            writeln!(f, "}}").into_diagnostic()?;
            writeln!(f).into_diagnostic()?;
        }
    }

    // Write paginated operations.
    for (name, shape) in &model.shapes {
        if let schema::Shape::Operation(shape) = shape {
            let Some(paginated) = &shape.traits.paginated else {
                continue;
            };
            let name = &name.name;
            let input = shape_id_to_ts(&shape.input.target);
            let output = shape_id_to_ts(&shape.output.target);

            let input_token = paginated
                .input_token
                .as_deref()
                .or_else(|| service.traits.paginated.as_ref()?.input_token.as_deref())
                .ok_or(miette::diagnostic!(
                    "no input token in paginated config for {name}"
                ))?;
            let output_token = paginated
                .output_token
                .as_deref()
                .or_else(|| service.traits.paginated.as_ref()?.output_token.as_deref())
                .ok_or(miette::diagnostic!(
                    "no output token in paginated config for {name}"
                ))?
                .replace('.', "?."); // e.g. output.EngineDefaults?.Marker;

            writeln!(f, "export async function* paginate{name}(").into_diagnostic()?;
            writeln!(f, "  clientConfig: ClientConfig,").into_diagnostic()?;
            writeln!(f, "  input: {input},").into_diagnostic()?;
            writeln!(f, "): AsyncIterable<{output}> {{").into_diagnostic()?;
            writeln!(f, "  let token: {input}[{input_token:?}] = undefined;").into_diagnostic()?;
            writeln!(f, "  do {{").into_diagnostic()?;
            writeln!(
                f,
                "    const output: {output} = await {name}(clientConfig, {{ ...input, {input_token}: token }});",
            ).into_diagnostic()?;
            writeln!(f, "    token = output.{output_token};").into_diagnostic()?;
            writeln!(f, "    yield output;").into_diagnostic()?;
            writeln!(f, "  }} while (token);").into_diagnostic()?;
            writeln!(f, "}}").into_diagnostic()?;
            writeln!(f).into_diagnostic()?;
        }
    }

    // Write all types.
    for (name, shape) in &model.shapes {
        match shape {
            schema::Shape::Service(_)
            | schema::Shape::Operation(_)
            | schema::Shape::Resource(_) => {}
            schema::Shape::Structure(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                writeln!(f, "export interface {} {{", name.name).into_diagnostic()?;
                for (member_name, member) in &shape.members {
                    doc_comment(&mut f, "    ", &member.traits.documentation)?;
                    writeln!(
                        f,
                        "    readonly {member_name}{}: {};",
                        if member.traits.required.is_some() {
                            ""
                        } else {
                            "?"
                        },
                        shape_id_to_ts(&member.target)
                    )
                    .into_diagnostic()?;
                }
                writeln!(f, "}}").into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::List(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                writeln!(
                    f,
                    "export type {} = ReadonlyArray<{}>;",
                    name.name,
                    shape_id_to_ts(&shape.member.target),
                )
                .into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Map(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                writeln!(
                    f,
                    "export type {id} = {{ readonly [key: string]: {value} }}; // key: {key}",
                    id = name.name,
                    key = shape_id_to_ts(&shape.key.target),
                    value = shape_id_to_ts(&shape.value.target),
                )
                .into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Union(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                writeln!(f, "export type {} = // union", name.name).into_diagnostic()?;
                for (name, member) in &shape.members {
                    doc_comment(&mut f, "    ", &member.traits.documentation)?;
                    writeln!(
                        f,
                        "    | {{ readonly {name}: {ty}{others} }}",
                        ty = shape_id_to_ts(&member.target),
                        others = shape.members.keys().filter(|&k| k != name).fold(
                            String::new(),
                            |mut acc, n| {
                                use std::fmt::Write;
                                write!(acc, ", readonly {n}?: never")
                                    .expect("write to String should not fail");
                                acc
                            }
                        ),
                    )
                    .into_diagnostic()?;
                }
                writeln!(f, "    ;").into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Enum(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                writeln!(f, "export type {} = // enum", name.name).into_diagnostic()?;
                for value in shape.members.values() {
                    writeln!(f, "    | {value:?}", value = &value.traits.enum_value)
                        .into_diagnostic()?;
                }
                writeln!(f, "    ;").into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Boolean(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                if name.name == "boolean" {
                    write!(f, "// ").into_diagnostic()?;
                }
                writeln!(f, "export type {} = boolean;", name.name).into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Integer(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                if name.name == "number" {
                    write!(f, "// ").into_diagnostic()?;
                }
                writeln!(f, "export type {} = number; // i32", name.name).into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Long(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                if name.name == "number" {
                    write!(f, "// ").into_diagnostic()?;
                }
                writeln!(f, "export type {} = number; // i64", name.name).into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Float(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                if name.name == "number" {
                    write!(f, "// ").into_diagnostic()?;
                }
                writeln!(f, "export type {} = number; // f32", name.name).into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Double(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                if name.name == "number" {
                    write!(f, "// ").into_diagnostic()?;
                }
                writeln!(f, "export type {} = number; // f64", name.name).into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::String(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                if name.name == "string" {
                    write!(f, "// ").into_diagnostic()?;
                }
                if let Some(enum_) = &shape.traits.enum_ {
                    writeln!(f, "export type {} =", name.name).into_diagnostic()?;
                    for item in enum_.iter() {
                        doc_comment(&mut f, "    ", &item.documentation)?;
                        writeln!(f, "    | {:?}", item.value).into_diagnostic()?;
                    }
                    writeln!(f, "    ;").into_diagnostic()?;
                } else {
                    writeln!(f, "export type {} = string;", name.name).into_diagnostic()?;
                }
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Blob(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                writeln!(f, "export type {} = string; // Blob", name.name).into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Timestamp(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                if name.name == "Date" {
                    write!(f, "// ").into_diagnostic()?;
                }
                writeln!(
                    f,
                    "export type {} = Date; // Timestamp {:?}",
                    name.name, shape.traits.timestamp_format
                )
                .into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
            schema::Shape::Document(shape) => {
                doc_comment(&mut f, "", &shape.traits.documentation)?;
                writeln!(f, "export type {} = string; // Document", name.name).into_diagnostic()?;
                writeln!(f).into_diagnostic()?;
            }
        }
    }

    fn doc_comment(file: &mut impl Write, indent: &str, value: &Option<String>) -> Result<()> {
        let Some(mut doc) = value.as_deref() else {
            return Ok(());
        };

        let doc_without_first_p;
        if let Some(doc_after_p) = doc.strip_prefix("<p>") {
            doc_without_first_p = doc_after_p.replacen("</p>", "", 1);
            doc = &doc_without_first_p;
        }

        let doc_without_comment_close;
        if doc.contains("*/") {
            doc_without_comment_close = doc.replace("*/", "*&#47;");
            doc = &doc_without_comment_close;
        }

        if indent.len() + 4 + doc.len() + 3 < 80 {
            writeln!(file, "{indent}/** {} */", doc.trim(), indent = indent).into_diagnostic()?;
            return Ok(());
        }

        // Word wrap at 120 chars, including indent and " * " decoration, but
        // don't go under 50 chars if the indent is really big.
        let wrap_len = (120 - indent.len() - 3).max(50);
        writeln!(file, "{indent}/**").into_diagnostic()?;
        for mut line in doc.lines() {
            while line.len() > wrap_len {
                // wrap_len might not be on a character boundary
                let mut char_wrap_len = wrap_len;
                while !line.is_char_boundary(char_wrap_len) {
                    char_wrap_len -= 1;
                }

                // Find the last space before char_wrap_len, or the first space after wrap_len if
                // none, or break out to write the remaining text if no spaces remain.
                let Some(line_wrap_len) = line[..char_wrap_len]
                    .rfind(' ')
                    .or_else(|| line[char_wrap_len..].find(' ').map(|i| i + char_wrap_len))
                else {
                    break;
                };
                writeln!(file, "{indent} * {}", line[..line_wrap_len].trim()).into_diagnostic()?;
                line = line[line_wrap_len..].trim_start();
            }
            writeln!(file, "{indent} * {line}", line = line.trim()).into_diagnostic()?;
        }
        writeln!(file, "{indent} */").into_diagnostic()?;
        Ok(())
    }

    fn shape_id_to_ts(shape: &schema::ShapeId) -> &str {
        match shape.name.as_str() {
            "Blob" => "Uint8Array",
            "Boolean" => "boolean",
            "Document" => "string",
            "Double" => "number",
            "Float" => "number",
            "Integer" => "number",
            "Long" => "number",
            "PrimitiveBoolean" => "boolean",
            "PrimitiveLong" => "number",
            "String" => "string",
            "Timestamp" => "string",
            "Unit" => "void",
            _ => &shape.name,
        }
    }

    Ok(())
}
