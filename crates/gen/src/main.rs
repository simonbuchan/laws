#![allow(dead_code)]

use clap::{Parser, Subcommand};
use std::collections::HashMap;
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
        #[clap(short, long)]
        minimal: bool,
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
        Some(Command::DumpEndpointRules { name, minimal }) => {
            let model_path = models_path.join(format!("{}.json", name));
            let model = parse_model(&model_path)?;
            dump_endpoint_rules(
                &model,
                if minimal {
                    EndpointRulesFilter::minimal()
                } else {
                    EndpointRulesFilter::default()
                },
            )?;
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

    let results = fs::read_dir(models_path)
        .into_diagnostic()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            if entry.file_name().to_string_lossy().ends_with(".json") {
                Some(entry)
            } else {
                None
            }
        })
        .par_bridge()
        .into_par_iter()
        .map(|entry| -> bool {
            let model = match parse_model(&entry.path()) {
                Ok(model) => model,
                Err(error) => {
                    render_report(&error);
                    return false;
                }
            };

            if let Err(error) = fs::create_dir_all(ts_services_dir_path) {
                eprintln!("  {error} while creating {ts_services_dir_path:?}");
                return false;
            }

            let mut ts_service_path = ts_services_dir_path.join(entry.file_name());
            ts_service_path.set_extension("ts");
            if let Err(error) = laws_write_ts::write_service(&model, &ts_service_path)
                .wrap_err_with(|| format!("writing {ts_service_path:?}"))
            {
                render_report(&error);
                if let Err(error) = fs::remove_file(&ts_service_path) {
                    eprintln!("  {error} while removing {ts_service_path:?}");
                }
                false
            } else {
                const ANSI_GREEN: &str = "\x1b[32m";
                const ANSI_RESET: &str = "\x1b[0m";
                const TICK: &str = "\u{2713}";
                println!("  {ANSI_GREEN}{TICK}{ANSI_RESET} wrote {ts_service_path:?}");
                true
            }
        })
        .collect::<Vec<_>>();

    let mut success = 0;
    let mut total = 0;
    for result in results {
        total += 1;
        if result {
            success += 1;
        }
    }
    println!("wrote {success} / {total} services");

    Ok(())
}

fn render_report(report: &miette::Report) {
    // It's way too hard to get miette to render a graphical report
    let handler = miette::GraphicalReportHandler::new();
    let mut output = String::new();
    if let Err(error) = handler.render_report(&mut output, report.as_ref()) {
        std::eprintln!("error rendering report: {error}");
    } else {
        std::eprint!("{output}");
    }
}

#[derive(Clone)]
enum EndpointRuleValue {
    Unknown,
    Invalid,
    Required,
    ConstUnset,
    ConstNull,
    ConstBool(bool),
    ConstString(String),
    ConstObject(HashMap<String, EndpointRuleValue>),
}

impl EndpointRuleValue {
    fn is_set(&self) -> EndpointRuleValue {
        match self {
            EndpointRuleValue::Unknown => EndpointRuleValue::Unknown,
            EndpointRuleValue::Invalid => EndpointRuleValue::Invalid,
            EndpointRuleValue::ConstUnset => EndpointRuleValue::ConstBool(false),
            EndpointRuleValue::Required
            | EndpointRuleValue::ConstNull
            | EndpointRuleValue::ConstBool(_)
            | EndpointRuleValue::ConstString(_)
            | EndpointRuleValue::ConstObject(_) => EndpointRuleValue::ConstBool(true),
        }
    }
}

#[derive(Clone, Default)]
struct EndpointRulesFilter {
    values: HashMap<String, EndpointRuleValue>,
    no_arn_bucket: bool,
}

impl EndpointRulesFilter {
    fn minimal() -> Self {
        let mut values = HashMap::new();
        values.insert("Region".to_string(), EndpointRuleValue::Required);
        values.insert("Bucket".to_string(), EndpointRuleValue::Required);
        values.insert("UseFIPS".to_string(), EndpointRuleValue::ConstBool(false));
        values.insert(
            "UseDualStack".to_string(),
            EndpointRuleValue::ConstBool(false),
        );
        values.insert(
            "Accelerate".to_string(),
            EndpointRuleValue::ConstBool(false),
        );
        values.insert(
            "ForcePathStyle".to_string(),
            EndpointRuleValue::ConstBool(false),
        );
        values.insert(
            "UseArnRegion".to_string(),
            EndpointRuleValue::ConstBool(false),
        );
        values.insert(
            "UseGlobalEndpoint".to_string(),
            EndpointRuleValue::ConstBool(false),
        );
        values.insert("Endpoint".to_string(), EndpointRuleValue::ConstUnset);
        let no_arn_bucket = true;
        Self {
            values,
            no_arn_bucket,
        }
    }
}

struct EndpointRuleEvaluation<'rule> {
    always_false: bool,
    unknown_conditions: Vec<&'rule schema::EndpointRuleCondition>,
    child_filter: EndpointRulesFilter,
}

impl EndpointRulesFilter {
    fn evaluate_rule<'rule>(
        &self,
        rule: &'rule schema::EndpointRuleItem,
    ) -> EndpointRuleEvaluation<'rule> {
        let mut result = EndpointRuleEvaluation {
            always_false: false,
            unknown_conditions: vec![],
            child_filter: self.clone(),
        };

        for condition in &rule.conditions {
            let value = self.condition_const(condition);

            if let Some(name) = condition.assign.clone() {
                result.child_filter.values.insert(name, value);
                // assignments are not conditions
                continue;
            }

            match value {
                EndpointRuleValue::ConstBool(true) => {}
                EndpointRuleValue::ConstBool(false) => {
                    result.always_false = true;
                    return result;
                }
                _ => {
                    result.unknown_conditions.push(condition);
                }
            }
        }

        result
    }

    // conditions that are always true can be removed
    fn exclude_condition(&self, condition: &schema::EndpointRuleCondition) -> bool {
        matches!(
            self.condition_const(condition),
            EndpointRuleValue::ConstBool(true)
        )
    }

    fn condition_const(&self, condition: &schema::EndpointRuleCondition) -> EndpointRuleValue {
        match &condition.function {
            schema::EndpointRuleConditionFn::Not(e) => match self.expr_const(e) {
                EndpointRuleValue::Unknown => EndpointRuleValue::Unknown,
                EndpointRuleValue::ConstBool(value) => EndpointRuleValue::ConstBool(!value),
                _ => EndpointRuleValue::Invalid,
            },
            schema::EndpointRuleConditionFn::BooleanEquals(l, r) => {
                match (self.expr_const(l), self.expr_const(r)) {
                    (EndpointRuleValue::Unknown, _) => EndpointRuleValue::Unknown,
                    (_, EndpointRuleValue::Unknown) => EndpointRuleValue::Unknown,
                    (EndpointRuleValue::ConstBool(l), EndpointRuleValue::ConstBool(r)) => {
                        EndpointRuleValue::ConstBool(l == r)
                    }
                    _ => EndpointRuleValue::Invalid,
                }
            }
            schema::EndpointRuleConditionFn::StringEquals(l, r) => {
                match (self.expr_const(l), self.expr_const(r)) {
                    (EndpointRuleValue::Unknown, _) => EndpointRuleValue::Unknown,
                    (_, EndpointRuleValue::Unknown) => EndpointRuleValue::Unknown,
                    (EndpointRuleValue::ConstString(l), EndpointRuleValue::ConstString(r)) => {
                        EndpointRuleValue::ConstBool(l == r)
                    }
                    _ => EndpointRuleValue::Invalid,
                }
            }
            schema::EndpointRuleConditionFn::IsSet(e) => self.expr_const(e).is_set(),
            schema::EndpointRuleConditionFn::Substring(expr, start, end, reverse) => {
                match self.expr_const(expr) {
                    EndpointRuleValue::Unknown => EndpointRuleValue::Unknown,
                    EndpointRuleValue::ConstString(value) => {
                        if !value.is_ascii() {
                            return EndpointRuleValue::ConstNull;
                        }
                        if *end < *start {
                            return EndpointRuleValue::Invalid;
                        }

                        let len = *end - *start;
                        let start = if *reverse { value.len() - *end } else { *start };
                        let end = start + len;
                        if value.len() < end {
                            EndpointRuleValue::ConstNull
                        } else {
                            EndpointRuleValue::ConstString(value[start..end].to_owned())
                        }
                    }
                    _ => EndpointRuleValue::Invalid,
                }
            }
            schema::EndpointRuleConditionFn::AwsParseArn(e) => {
                if !self.no_arn_bucket {
                    return EndpointRuleValue::Unknown;
                }
                match e {
                    schema::EndpointRuleExpr::Reference { name } if name == "Bucket" => {
                        EndpointRuleValue::ConstObject({
                            let mut map = HashMap::new();
                            map.insert(
                                "resourceId[0]".to_string(),
                                EndpointRuleValue::ConstString("".to_string()),
                            );
                            map
                        })
                    }
                    _ => EndpointRuleValue::Unknown,
                }
            }
            schema::EndpointRuleConditionFn::GetAttr(e, name) => match self.expr_const(e) {
                EndpointRuleValue::Unknown => EndpointRuleValue::Unknown,
                EndpointRuleValue::ConstObject(map) => {
                    map.get(name).cloned().unwrap_or(EndpointRuleValue::Unknown)
                }
                _ => EndpointRuleValue::Invalid,
            },
            schema::EndpointRuleConditionFn::ParseURL(..)
            | schema::EndpointRuleConditionFn::IsValidHostLabel(..)
            | schema::EndpointRuleConditionFn::UriEncode(..)
            | schema::EndpointRuleConditionFn::AwsPartition(..)
            | schema::EndpointRuleConditionFn::AwsIsVirtualHostableS3Bucket(..) => {
                EndpointRuleValue::Unknown
            }
        }
    }

    fn expr_const(&self, expr: &schema::EndpointRuleExpr) -> EndpointRuleValue {
        match expr {
            schema::EndpointRuleExpr::Boolean(s) => EndpointRuleValue::ConstBool(*s),
            schema::EndpointRuleExpr::String(s) => EndpointRuleValue::ConstString(s.clone()),
            schema::EndpointRuleExpr::Condition(cond) => self.condition_const(cond),
            schema::EndpointRuleExpr::Reference { name } => self
                .values
                .get(name)
                .cloned()
                .unwrap_or(EndpointRuleValue::Unknown),
        }
    }
}

fn dump_endpoint_rules(model: &schema::Model, filter: EndpointRulesFilter) -> Result<()> {
    let service = model
        .shapes
        .values()
        .find_map(|shape| match shape {
            schema::Shape::Service(service) => Some(service),
            _ => None,
        })
        .ok_or(miette::diagnostic!("no service shape found in model"))?;
    let schema::EndpointRuleSetTrait::V1_0(rule_set) = &service.traits.endpoint_rule_set;

    filtered_rules(0, &rule_set.rules, filter);

    fn filtered_rules(
        indent: usize,
        rules: &[schema::EndpointRuleItem],
        filter: EndpointRulesFilter,
    ) {
        for rule in rules {
            let evaluation = filter.evaluate_rule(rule);
            if evaluation.always_false {
                continue;
            }
            print_rule_item(indent + 2, rule, &evaluation);
        }
    }

    fn print_rule_item(
        indent: usize,
        item: &schema::EndpointRuleItem,
        evaluation: &EndpointRuleEvaluation,
    ) {
        let mut has_condition = false;
        print!("{:indent$}", "");
        for condition in &evaluation.unknown_conditions {
            if !has_condition {
                has_condition = true;
                print!("if ");
            } else {
                print!(" and\n{:indent$}   ", "");
            }
            print_condition(indent, condition);
        }
        if !has_condition {
            print!("else");
        }
        print!(" => ");
        match &item.rule {
            schema::EndpointRule::Error { error } => {
                print!("error: {error}");
            }
            schema::EndpointRule::Tree { rules } => {
                println!("{{");
                filtered_rules(indent + 2, rules, evaluation.child_filter.clone());
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
