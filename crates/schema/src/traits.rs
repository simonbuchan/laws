use indexmap::IndexMap;

use crate::shape_id::ShapeId;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArnReferenceTrait {
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(default)]
    pub service: Option<String>,
    #[serde(rename = "type", default)]
    pub type_: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataTrait {
    Account,
    Tagging,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorTrait {
    Client,
    Server,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientEndpointDiscoveryTrait {
    pub operation: String,
    pub error: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlPlaneTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataPlaneTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfoTrait {
    pub sdk_id: String,
    #[serde(default)]
    pub arn_namespace: Option<String>,
    #[serde(default)]
    pub cloud_formation_name: Option<String>,
    #[serde(default)]
    pub cloud_trail_event_source: Option<String>,
    #[serde(default)]
    pub endpoint_prefix: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TagEnabledTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sigv4Trait {
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IamConditionKeyDef {
    #[serde(rename = "type")]
    pub type_: String,
    pub documentation: String,
    #[serde(default)]
    pub external_documentation: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AwsJson1_0Trait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AwsJson1_1Trait {
    #[serde(default)]
    pub http: Vec<HttpVersion>,
    #[serde(default)]
    pub event_stream_http: Vec<HttpVersion>,
}

#[derive(Debug, serde::Deserialize)]
pub enum HttpVersion {
    #[serde(rename = "http/1.1")]
    Http1_1,
    #[serde(rename = "h2")]
    H2,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AwsQueryTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AwsQueryCompatibleTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AwsQueryErrorTrait {
    pub code: String,
    pub http_response_code: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ec2QueryTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestJson1Trait {
    #[serde(default)]
    pub http: Vec<HttpVersion>,
    #[serde(default)]
    pub event_stream_http: Vec<HttpVersion>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestXmlTrait {
    #[serde(default)]
    pub no_error_wrapping: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecatedTrait {
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub since: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CorsTrait {
    #[serde(default)]
    pub additional_allowed_headers: Vec<String>,
    #[serde(default)]
    pub additional_exposed_headers: Vec<String>,
    #[serde(default)]
    pub max_age: Option<u32>,
    #[serde(default)]
    pub origin: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpBearerAuthTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnstableTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XmlAttributeTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XmlNamespaceTrait {
    pub uri: String,
    #[serde(default)]
    pub prefix: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "version")]
pub enum EndpointRuleSetTrait {
    #[serde(rename = "1.0")]
    V1_0(crate::endpoint_rules::EndpointRuleSet),
}

#[derive(Debug, serde::Deserialize)]
pub struct EndpointTestsTrait {
    // ...
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientContextParamDef {
    pub documentation: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientDiscoveredEndpointTrait {
    #[serde(default)]
    pub required: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HttpChecksumTrait {
    #[serde(default)]
    pub request_algorithm_member: Option<String>,
    #[serde(default)]
    pub request_checksum_required: Option<bool>,
    #[serde(default)]
    pub request_validation_mode_member: Option<String>,
    #[serde(default)]
    pub response_algorithms: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointTrait {
    pub host_prefix: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalAuthTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpTrait {
    pub uri: String,
    pub method: String,
    #[serde(default)]
    pub code: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpChecksumRequiredTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdempotentTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadonlyTrait {}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaginatedTrait {
    #[serde(default)]
    pub input_token: Option<String>,
    #[serde(default)]
    pub output_token: Option<String>,
    #[serde(default)]
    pub page_size: Option<String>,
    #[serde(default)]
    pub items: Option<String>,
    #[serde(default)]
    pub max_results: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitableTrait {
    // ...
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnsignedPayloadTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct S3UnwrappedXmlOutputTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StaticParam {
    pub value: serde_json::Value,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Example {
    pub title: String,
    #[serde(default)]
    pub documentation: Option<String>,
    #[serde(default)]
    pub input: Option<serde_json::Value>,
    #[serde(default)]
    pub output: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArnTrait {
    pub template: String,
    #[serde(default)]
    pub absolute: bool,
    #[serde(default)]
    pub no_account: bool,
    #[serde(default)]
    pub no_region: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaggableTrait {
    pub property: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CloudformationResourceTrait {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub additional_schemas: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IamDisableConditionKeyInferenceTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IamResourceTrait {
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoReplaceTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudformationAdditionalIdentifierTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudformationExcludePropertyTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum CloudformationMutabilityTrait {
    CreateAndRead,
    Full,
    Read,
    Write,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddedDefaultTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientOptionalTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetryableTrait {
    #[serde(default)]
    pub throttling: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventPayloadTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostLabelTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpLabelTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpQueryParamsTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpResponseCodeTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdempotencyTokenTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiresLengthTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SensitiveTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropertyTrait {
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RangeTrait {
    #[serde(default)]
    pub min: Option<serde_json::Value>,
    #[serde(default)]
    pub max: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecommendedTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LengthTrait {
    #[serde(default)]
    pub min: Option<u32>,
    #[serde(default)]
    pub max: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NestedPropertiesTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotPropertyTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpPayloadTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XmlFlattenedTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum TimestampFormatTrait {
    EpochSeconds,
    DateTime,
    HttpDate,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextParamTrait {
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub resource: ShapeId,
    #[serde(default)]
    pub ids: IndexMap<String, String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UniqueItemsTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SparseTrait {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamingTrait {}
