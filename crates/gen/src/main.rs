#![allow(dead_code)]

use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

use miette::{IntoDiagnostic, Result, WrapErr};
use rayon::prelude::*;

use laws_schema as schema;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    FetchModels,
    WriteTs,
    DumpEndpointRules {
        #[clap(name = "SERVICE")]
        name: String,
    },
}

fn main() -> Result<()> {
    let models_path = Path::new("aws-models");
    let ts_services_dir_path = Path::new("ts-client/src/services");

    let args = Args::parse();
    if matches!(args.command, Some(Command::FetchModels)) || !models_path.exists() {
        println!("fetching models to {}", models_path.display());
        laws_fetch_models::fetch_models(models_path).wrap_err("fetching models")?;
    }

    match args.command {
        Some(Command::FetchModels) => {
            // already handled.
        }
        Some(Command::DumpEndpointRules { name }) => {
            let model_path = models_path.join(format!("{}.json", name));
            let model = parse_model(&model_path)?;
            dump_endpoint_rules(&model, &EndpointRulesFilter::default())?;
        }
        None | Some(Command::WriteTs) => {
            write_ts(models_path, ts_services_dir_path)?;
        }
    }

    Ok(())
}

fn parse_model(path: &Path) -> Result<schema::Model> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("reading {path:?}"))?;

    let model: schema::Model = schema::parse_model(&source)
        .into_diagnostic()
        .wrap_err_with(|| format!("parsing {path:?}"))?;

    Ok(model)
}

fn write_ts(models_path: &Path, ts_services_dir_path: &Path) -> Result<()> {
    println!("writing ts services to {ts_services_dir_path:?}");
    match fs::remove_dir_all(ts_services_dir_path) {
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        result => {
            result
                .into_diagnostic()
                .wrap_err("removing ts-client/services")?;
        }
    };

    fs::read_dir(models_path)
        .into_diagnostic()?
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| -> Result<()> {
            let entry = entry.into_diagnostic()?;
            if entry.file_type().into_diagnostic()?.is_file() {
                let model_path = entry.path();
                if model_path
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or_default()
                {
                    let model = parse_model(&model_path)?;

                    fs::create_dir_all(ts_services_dir_path).into_diagnostic()?;

                    let mut ts_service_path = ts_services_dir_path.join(entry.file_name());
                    ts_service_path.set_extension("ts");
                    if let Err(error) = laws_write_ts::write_service(&model, &ts_service_path)
                        .wrap_err_with(|| format!("writing {ts_service_path:?}"))
                    {
                        // It's way too hard to get miette to render a graphical report
                        let handler = miette::GraphicalReportHandler::new();
                        let mut output = String::new();
                        handler
                            .render_report(&mut output, error.as_ref())
                            .into_diagnostic()?;
                        std::eprint!("{output}");
                        fs::remove_file(&ts_service_path).into_diagnostic()?;
                    } else {
                        const ANSI_GREEN: &str = "\x1b[32m";
                        const ANSI_RESET: &str = "\x1b[0m";
                        const TICK: &str = "\u{2713}";
                        println!("  {ANSI_GREEN}{TICK}{ANSI_RESET} wrote {ts_service_path:?}");
                    }
                }
            }
            Ok(())
        })?;

    Ok(())
}

#[derive(Default)]
struct EndpointRulesFilter {
    // builtins
    region: Option<String>,
    use_fips: Option<bool>,
    use_dualstack: Option<bool>,
    custom_endpoint: Option<String>,
}

fn dump_endpoint_rules(model: &schema::Model, filter: &EndpointRulesFilter) -> Result<()> {
    let service = model
        .shapes
        .values()
        .find_map(|shape| match shape {
            schema::Shape::Service(service) => Some(service),
            _ => None,
        })
        .ok_or(miette::diagnostic!("no service shape found in model"))?;
    let schema::EndpointRuleSetTrait::V1_0(rule_set) = &service.traits.endpoint_rule_set;

    for rule in &rule_set.rules {
        print_rule_item(0, rule);
    }

    fn print_rule_item(indent: usize, item: &schema::EndpointRuleItem) {
        print!("{:indent$}", "");
        if !item.conditions.is_empty() {
            print!("if ");
            let mut first = true;
            for condition in &item.conditions {
                if first {
                    first = false;
                } else {
                    print!(" and\n{:indent$}   ", "");
                }
                print_condition(indent, condition);
            }
        } else {
            print!("else");
        }
        print!(" => ");
        match &item.rule {
            schema::EndpointRule::Error { error } => {
                print!("error: {error}");
            }
            schema::EndpointRule::Tree { rules } => {
                println!("{{");
                for rule in rules {
                    print_rule_item(indent + 2, rule);
                }
                print!("{:indent$}}}", "");
            }
            schema::EndpointRule::Endpoint { endpoint } => {
                print!("endpoint: ");
                print_expr(indent, &endpoint.url);
                if endpoint.properties.backend.is_some()
                    || !endpoint.properties.auth_schemes.is_empty()
                    || !endpoint.headers.is_empty()
                {
                    println!(" {{");
                    if let Some(backend) = &endpoint.properties.backend {
                        println!("{:indent$}  backend: {backend}", "");
                    }
                    if !endpoint.properties.auth_schemes.is_empty() {
                        println!("{:indent$}  auth_schemes: [", "");
                        for auth_scheme in &endpoint.properties.auth_schemes {
                            match auth_scheme {
                                schema::EndpointAuthScheme::Sigv4 {
                                    signing_name,
                                    signing_region,
                                    disable_double_encoding,
                                } => {
                                    println!("{:indent$}    sigv4({signing_name}, {signing_region}, {disable_double_encoding})", "");
                                }
                                schema::EndpointAuthScheme::Sigv4a {
                                    signing_name,
                                    signing_region_set,
                                    disable_double_encoding,
                                } => {
                                    println!("{:indent$}    sigv4a({signing_name}, [{signing_region_set}], {disable_double_encoding})", "",
                                             signing_region_set = signing_region_set.join(", "));
                                }
                                schema::EndpointAuthScheme::Sigv4S3Express {
                                    signing_name,
                                    signing_region,
                                    disable_double_encoding,
                                } => {
                                    println!("{:indent$}    sigv4-s3express({signing_name}, {signing_region}, {disable_double_encoding})", "");
                                }
                            }
                        }
                        println!("{:indent$}  ]", "");
                    }
                    if !endpoint.headers.is_empty() {
                        println!("{:indent$}  headers: {{", "");
                        for (name, values) in &endpoint.headers {
                            for value in values {
                                println!("{:indent$}    {name}: {value}", "");
                            }
                        }
                        println!("{:indent$}  }}", "");
                    }
                    print!("{:indent$}}}", "");
                }
            }
        }
        println!();
    }

    fn print_expr(indent: usize, expr: &schema::EndpointRuleExpr) {
        match expr {
            schema::EndpointRuleExpr::Boolean(s) => print!("{s:?}"),
            schema::EndpointRuleExpr::String(s) => print!("{s:?}"),
            schema::EndpointRuleExpr::Condition(cond) => print_condition(indent, cond),
            schema::EndpointRuleExpr::Reference { name } => print!("${name}"),
        }
    }

    fn print_condition(indent: usize, condition: &schema::EndpointRuleCondition) {
        if let Some(assign) = &condition.assign {
            print!("let {assign} = ");
        }
        match &condition.function {
            schema::EndpointRuleConditionFn::Not(e) => {
                print!("not ");
                print_expr(indent + 4, e);
            }
            schema::EndpointRuleConditionFn::BooleanEquals(l, r) => {
                print!("boolean_equals(");
                print_expr(indent, l);
                print!(", ");
                print_expr(indent, r);
                print!(")");
            }
            schema::EndpointRuleConditionFn::StringEquals(l, r) => {
                print!("string_equals(");
                print_expr(indent, l);
                print!(", ");
                print_expr(indent, r);
                print!(")");
            }
            schema::EndpointRuleConditionFn::GetAttr(e, name) => {
                print!("(");
                print_expr(indent, e);
                print!(").{name}");
            }
            schema::EndpointRuleConditionFn::IsSet(e) => {
                print!("is_set(");
                print_expr(indent, e);
                print!(")");
            }
            schema::EndpointRuleConditionFn::ParseURL(e) => {
                print!("parse_url(");
                print_expr(indent, e);
                print!(")");
            }
            schema::EndpointRuleConditionFn::IsValidHostLabel(expr, allow_underscores) => {
                print!("is_valid_host_label(");
                print_expr(indent, expr);
                print!(", ");
                print!("{allow_underscores}");
                print!(")");
            }
            schema::EndpointRuleConditionFn::Substring(expr, start, end, from_end) => {
                print!("substring(");
                print_expr(indent, expr);
                print!(", {start}, {end}, {from_end})");
            }
            schema::EndpointRuleConditionFn::UriEncode(expr) => {
                print!("uri_encode(");
                print_expr(indent, expr);
                print!(")");
            }
            schema::EndpointRuleConditionFn::AwsPartition(expr) => {
                print!("aws_partition(");
                print_expr(indent, expr);
                print!(")");
            }
            schema::EndpointRuleConditionFn::AwsParseArn(expr) => {
                print!("aws_parse_arn(");
                print_expr(indent, expr);
                print!(")");
            }
            schema::EndpointRuleConditionFn::AwsIsVirtualHostableS3Bucket(expr, foo) => {
                print!("aws_is_virtual_hostable_s3_bucket(");
                print_expr(indent, expr);
                print!(", {foo})");
            }
        }
    }

    Ok(())
}
