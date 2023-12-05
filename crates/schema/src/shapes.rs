use indexmap::IndexMap;

use crate::shape_id::ShapeId;
use crate::traits::*;
use crate::ShapeRef;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum Shape {
    Service(Box<ServiceShape>),
    Operation(Box<OperationShape>),
    Resource(Box<ResourceShape>),
    Structure(Box<StructureShape>),

    List(Box<ListShape>),
    Map(Box<MapShape>),
    Union(Box<UnionShape>),
    Enum(Box<EnumShape>),

    Boolean(Box<BooleanShape>),
    Integer(Box<IntegerShape>),
    Long(Box<LongShape>),
    Float(Box<FloatShape>),
    Double(Box<DoubleShape>),
    String(Box<StringShape>),
    Blob(Box<BlobShape>),

    Timestamp(Box<TimestampShape>),
    Document(Box<DocumentShape>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BooleanShape {
    #[serde(default)]
    pub traits: BooleanTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BooleanTraits {
    #[serde(rename = "smithy.api#default", default)]
    pub default: Option<bool>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegerShape {
    #[serde(default)]
    pub traits: IntegerTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegerTraits {
    #[serde(rename = "smithy.api#default", default)]
    pub default: Option<i32>,
    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#range", default)]
    pub range: Option<RangeTrait>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LongShape {
    #[serde(default)]
    pub traits: LongTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LongTraits {
    #[serde(rename = "smithy.api#default", default)]
    pub default: Option<i64>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#range", default)]
    pub range: Option<RangeTrait>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FloatShape {
    #[serde(default)]
    pub traits: FloatTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FloatTraits {
    #[serde(rename = "smithy.api#default", default)]
    pub default: Option<f32>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#range", default)]
    pub range: Option<RangeTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DoubleShape {
    #[serde(default)]
    pub traits: DoubleTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DoubleTraits {
    #[serde(rename = "smithy.api#default", default)]
    pub default: Option<f64>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#range", default)]
    pub range: Option<RangeTrait>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceShape {
    pub version: String,
    #[serde(default)]
    pub operations: Vec<ShapeRef>,
    #[serde(default)]
    pub resources: Vec<ShapeRef>,
    #[serde(default)]
    pub errors: Vec<ShapeRef>,
    pub traits: ServiceTraits,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StringShape {
    #[serde(default)]
    pub traits: StringTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StringTraits {
    #[serde(rename = "aws.api#arnReference", default)]
    pub arn_reference: Option<ArnReferenceTrait>,
    #[serde(rename = "aws.api#data", default)]
    pub data: Option<DataTrait>,

    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#externalDocumentation", default)]
    pub external_documentation: Option<IndexMap<String, String>>,
    #[serde(rename = "smithy.api#enum", default)]
    pub enum_: Option<Vec<StringEnumItem>>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#mediaType", default)]
    pub media_type: Option<String>,
    #[serde(rename = "smithy.api#pattern", default)]
    pub pattern: Option<String>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlobShape {
    #[serde(default)]
    pub traits: BlobTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlobTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#mediaType", default)]
    pub media_type: Option<String>,
    #[serde(rename = "smithy.api#requiresLength", default)]
    pub requires_length: Option<RequiresLengthTrait>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
    #[serde(rename = "smithy.api#streaming", default)]
    pub streaming: Option<StreamingTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimestampShape {
    #[serde(default)]
    pub traits: TimestampTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimestampTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#timestampFormat", default)]
    pub timestamp_format: Option<TimestampFormatTrait>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentShape {
    #[serde(default)]
    pub traits: DocumentTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StringEnumItem {
    pub value: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub documentation: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceTraits {
    #[serde(rename = "aws.api#clientEndpointDiscovery", default)]
    pub client_endpoint_discovery: Option<ClientEndpointDiscoveryTrait>,
    #[serde(rename = "aws.api#controlPlane", default)]
    pub control_plane: Option<ControlPlaneTrait>,
    #[serde(rename = "aws.api#dataPlane", default)]
    pub data_plane: Option<DataPlaneTrait>,
    #[serde(rename = "aws.api#service")]
    pub service: ServiceInfoTrait,
    #[serde(rename = "aws.api#tagEnabled")]
    pub tag_enabled: Option<TagEnabledTrait>,

    #[serde(rename = "aws.auth#sigv4", default)]
    pub auth_sigv4: Option<Sigv4Trait>,

    #[serde(rename = "aws.iam#defineConditionKeys", default)]
    pub iam_define_condition_keys: IndexMap<String, IamConditionKeyDef>,
    #[serde(rename = "aws.iam#supportedPrincipalTypes", default)]
    pub iam_supported_principal_types: Vec<String>,

    #[serde(rename = "aws.protocols#awsJson1_0", default)]
    pub protocols_aws_json_1_0: Option<AwsJson1_0Trait>,
    #[serde(rename = "aws.protocols#awsJson1_1", default)]
    pub protocols_aws_json_1_1: Option<AwsJson1_1Trait>,
    #[serde(rename = "aws.protocols#awsQuery", default)]
    pub protocols_aws_query: Option<AwsQueryTrait>,
    #[serde(rename = "aws.protocols#awsQueryCompatible", default)]
    pub protocols_aws_query_compatible: Option<AwsQueryCompatibleTrait>,
    #[serde(rename = "aws.protocols#awsQueryError", default)]
    pub protocols_aws_query_error: Option<AwsQueryErrorTrait>,
    #[serde(rename = "aws.protocols#ec2Query", default)]
    pub protocols_ec2_query: Option<Ec2QueryTrait>,
    #[serde(rename = "aws.protocols#restJson1", default)]
    pub protocols_rest_json_1: Option<RestJson1Trait>,
    #[serde(rename = "aws.protocols#restXml", default)]
    pub protocols_rest_xml: Option<RestXmlTrait>,

    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#cors", default)]
    pub cors: Option<CorsTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#externalDocumentation", default)]
    pub external_documentation: Option<IndexMap<String, String>>,
    #[serde(rename = "smithy.api#httpBearerAuth", default)]
    pub http_bearer_auth: Option<HttpBearerAuthTrait>,
    #[serde(rename = "smithy.api#paginated", default)]
    pub paginated: Option<PaginatedTrait>,
    #[serde(rename = "smithy.api#suppress", default)]
    pub suppress: Vec<String>,
    #[serde(rename = "smithy.api#title")]
    pub title: String,
    #[serde(rename = "smithy.api#unstable", default)]
    pub unstable: Option<UnstableTrait>,
    #[serde(rename = "smithy.api#xmlNamespace", default)]
    pub xml_namespace: Option<XmlNamespaceTrait>,

    #[serde(rename = "smithy.rules#clientContextParams", default)]
    pub client_context_params: Option<IndexMap<String, ClientContextParamDef>>,
    #[serde(rename = "smithy.rules#endpointRuleSet")]
    pub endpoint_rule_set: EndpointRuleSetTrait,
    #[serde(rename = "smithy.rules#endpointTests")]
    pub endpoint_tests: EndpointTestsTrait,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationShape {
    pub input: ShapeRef,
    pub output: ShapeRef,
    #[serde(default)]
    pub errors: Vec<ShapeRef>,
    pub traits: OperationTraits,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationTraits {
    #[serde(rename = "aws.auth#unsignedPayload", default)]
    pub auth_unsigned_payload: Option<UnsignedPayloadTrait>,

    #[serde(rename = "aws.customizations#s3UnwrappedXmlOutput", default)]
    pub customizations_s3_unwrapped_xml_output: Option<S3UnwrappedXmlOutputTrait>,

    #[serde(rename = "aws.api#controlPlane", default)]
    pub control_plane: Option<ControlPlaneTrait>,
    #[serde(rename = "aws.api#dataPlane", default)]
    pub data_plane: Option<DataPlaneTrait>,

    #[serde(rename = "aws.api#clientDiscoveredEndpoint", default)]
    pub client_discovered_endpoint: Option<ClientDiscoveredEndpointTrait>,

    #[serde(rename = "aws.iam#actionName", default)]
    pub iam_action_name: Option<String>,
    #[serde(rename = "aws.iam#actionPermissionDescription", default)]
    pub iam_action_permission_description: Option<String>,
    #[serde(rename = "aws.iam#conditionKeys", default)]
    pub iam_condition_keys: Vec<String>,
    #[serde(rename = "aws.iam#requiredActions", default)]
    pub iam_required_actions: Vec<String>,

    #[serde(rename = "aws.protocols#httpChecksum", default)]
    pub http_checksum: Option<HttpChecksumTrait>,

    #[serde(rename = "smithy.api#auth", default)]
    pub auth: Vec<String>,
    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#externalDocumentation", default)]
    pub external_documentation: Option<IndexMap<String, String>>,
    #[serde(rename = "smithy.api#endpoint", default)]
    pub endpoint: Option<EndpointTrait>,
    #[serde(rename = "smithy.api#examples", default)]
    pub examples: Vec<Example>,
    #[serde(rename = "smithy.api#optionalAuth", default)]
    pub optional_auth: Option<OptionalAuthTrait>,
    #[serde(rename = "smithy.api#http", default)]
    pub http: Option<HttpTrait>,
    #[serde(rename = "smithy.api#httpChecksumRequired", default)]
    pub http_checksum_required: Option<HttpChecksumRequiredTrait>,
    #[serde(rename = "smithy.api#idempotent", default)]
    pub idempotent: Option<IdempotentTrait>,
    #[serde(rename = "smithy.api#readonly", default)]
    pub readonly: Option<ReadonlyTrait>,
    #[serde(rename = "smithy.api#suppress", default)]
    pub suppress: Vec<String>,
    #[serde(rename = "smithy.api#tags", default)]
    pub tags: Vec<String>,
    #[serde(rename = "smithy.api#paginated", default)]
    pub paginated: Option<PaginatedTrait>,

    #[serde(rename = "smithy.rules#staticContextParams", default)]
    pub static_context_params: Option<IndexMap<String, StaticParam>>,

    #[serde(rename = "smithy.waiters#waitable", default)]
    pub waitable: Option<WaitableTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceShape {
    #[serde(default)]
    pub identifiers: Option<IndexMap<String, ShapeRef>>,
    #[serde(default)]
    pub put: Option<ShapeRef>,
    #[serde(default)]
    pub create: Option<ShapeRef>,
    #[serde(default)]
    pub read: Option<ShapeRef>,
    #[serde(default)]
    pub update: Option<ShapeRef>,
    #[serde(default)]
    pub delete: Option<ShapeRef>,
    #[serde(default)]
    pub list: Option<ShapeRef>,
    #[serde(default)]
    pub collection_operations: Vec<ShapeRef>,
    #[serde(default)]
    pub operations: Vec<ShapeRef>,
    #[serde(default)]
    pub resources: Vec<ShapeRef>,
    #[serde(default)]
    pub properties: IndexMap<String, ShapeRef>,
    #[serde(default)]
    pub traits: ResourceTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceTraits {
    #[serde(rename = "aws.api#arn", default)]
    pub arn: Option<ArnTrait>,
    #[serde(rename = "aws.api#controlPlane", default)]
    pub control_plane: Option<ControlPlaneTrait>,
    #[serde(rename = "aws.api#dataPlane", default)]
    pub data_plane: Option<DataPlaneTrait>,
    #[serde(rename = "aws.api#taggable", default)]
    pub taggable: Option<TaggableTrait>,

    #[serde(rename = "aws.cloudformation#cfnResource", default)]
    pub cloudformation_cfn_resource: Option<CloudformationResourceTrait>,

    #[serde(rename = "aws.iam#conditionKeys", default)]
    pub iam_condition_keys: Vec<String>,
    #[serde(rename = "aws.iam#disableConditionKeyInference", default)]
    pub iam_disable_condition_key_inference: Option<IamDisableConditionKeyInferenceTrait>,
    #[serde(rename = "aws.iam#iamResource", default)]
    pub iam_resource: Option<IamResourceTrait>,

    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#externalDocumentation", default)]
    pub external_documentation: Option<IndexMap<String, String>>,
    #[serde(rename = "smithy.api#suppress", default)]
    pub suppress: Vec<String>,
    #[serde(rename = "smithy.api#noReplace", default)]
    pub no_replace: Option<NoReplaceTrait>,
    #[serde(rename = "smithy.api#unstable", default)]
    pub unstable: Option<UnstableTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructureShape {
    #[serde(default)]
    pub members: IndexMap<String, Member>,
    #[serde(default)]
    pub traits: StructureTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructureTraits {
    #[serde(rename = "aws.protocols#awsQueryError", default)]
    pub protocols_aws_query_error: Option<AwsQueryErrorTrait>,

    #[serde(rename = "aws.api#data", default)]
    pub data: Option<DataTrait>,

    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#externalDocumentation", default)]
    pub external_documentation: Option<IndexMap<String, String>>,
    #[serde(rename = "smithy.api#error", default)]
    pub error: Option<ErrorTrait>,
    #[serde(rename = "smithy.api#input", default)]
    pub input: Option<InputTrait>,
    #[serde(rename = "smithy.api#output", default)]
    pub output: Option<OutputTrait>,
    #[serde(rename = "smithy.api#httpError", default)]
    pub http_error: Option<u32>,
    #[serde(rename = "smithy.api#retryable", default)]
    pub retryable: Option<RetryableTrait>,
    #[serde(rename = "smithy.api#references", default)]
    pub references: Vec<Reference>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
    #[serde(rename = "smithy.api#xmlName", default)]
    pub xml_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub target: ShapeId,
    #[serde(default)]
    pub traits: MemberTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberTraits {
    #[serde(rename = "aws.cloudformation#cfnAdditionalIdentifier", default)]
    pub cloudformation_cfn_additional_identifier: Option<CloudformationAdditionalIdentifierTrait>,
    #[serde(rename = "aws.cloudformation#cfnExcludeProperty", default)]
    pub cloudformation_exclude_property: Option<CloudformationExcludePropertyTrait>,
    #[serde(rename = "aws.cloudformation#cfnMutability", default)]
    pub cloudformation_mutability: Option<CloudformationMutabilityTrait>,

    #[serde(rename = "aws.protocols#ec2QueryName", default)]
    pub ec2_query_name: Option<String>,

    #[serde(rename = "smithy.api#addedDefault", default)]
    pub added_default: Option<AddedDefaultTrait>,
    #[serde(rename = "smithy.api#clientOptional", default)]
    pub client_optional: Option<ClientOptionalTrait>,
    #[serde(rename = "smithy.api#default", default)]
    pub default: Option<serde_json::Value>,
    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#externalDocumentation", default)]
    pub external_documentation: Option<IndexMap<String, String>>,
    #[serde(rename = "smithy.api#eventPayload", default)]
    pub event_payload: Option<EventPayloadTrait>,
    #[serde(rename = "smithy.api#hostLabel", default)]
    pub host_label: Option<HostLabelTrait>,
    #[serde(rename = "smithy.api#httpLabel", default)]
    pub http_label: Option<HttpLabelTrait>,
    #[serde(rename = "smithy.api#httpHeader", default)]
    pub http_header: Option<String>,
    #[serde(rename = "smithy.api#httpPrefixHeaders", default)]
    pub http_prefix_headers: Option<String>,
    #[serde(rename = "smithy.api#httpPayload", default)]
    pub http_payload: Option<HttpPayloadTrait>,
    #[serde(rename = "smithy.api#httpQuery", default)]
    pub http_query: Option<String>,
    #[serde(rename = "smithy.api#httpQueryParams", default)]
    pub http_query_params: Option<HttpQueryParamsTrait>,
    #[serde(rename = "smithy.api#httpResponseCode", default)]
    pub http_response_code: Option<HttpResponseCodeTrait>,
    #[serde(rename = "smithy.api#idempotencyToken", default)]
    pub idempotency_token: Option<IdempotencyTokenTrait>,
    #[serde(rename = "smithy.api#jsonName", default)]
    pub json_name: Option<String>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#nestedProperties", default)]
    pub nested_properties: Option<NestedPropertiesTrait>,
    #[serde(rename = "smithy.api#notProperty", default)]
    pub not_property: Option<NotPropertyTrait>,
    #[serde(rename = "smithy.api#pattern", default)]
    pub pattern: Option<String>,
    #[serde(rename = "smithy.api#property", default)]
    pub property: Option<PropertyTrait>,
    #[serde(rename = "smithy.api#range", default)]
    pub range: Option<RangeTrait>,
    #[serde(rename = "smithy.api#recommended", default)]
    pub recommended: Option<RecommendedTrait>,
    #[serde(rename = "smithy.api#resourceIdentifier", default)]
    pub resource_identifier: Option<String>,
    #[serde(rename = "smithy.api#required", default)]
    pub required: Option<RequiredTrait>,
    #[serde(rename = "smithy.api#suppress", default)]
    pub suppress: Vec<String>,
    #[serde(rename = "smithy.api#tags", default)]
    pub tags: Vec<String>,
    #[serde(rename = "smithy.api#timestampFormat", default)]
    pub timestamp_format: Option<TimestampFormatTrait>,
    #[serde(rename = "smithy.api#unstable", default)]
    pub unstable: Option<UnstableTrait>,
    #[serde(rename = "smithy.api#xmlAttribute", default)]
    pub xml_attribute: Option<XmlAttributeTrait>,
    #[serde(rename = "smithy.api#xmlNamespace", default)]
    pub xml_namespace: Option<XmlNamespaceTrait>,
    #[serde(rename = "smithy.api#xmlName", default)]
    pub xml_name: Option<String>,
    #[serde(rename = "smithy.api#xmlFlattened", default)]
    pub xml_flattened: Option<XmlFlattenedTrait>,

    #[serde(rename = "smithy.rules#contextParam", default)]
    pub context_param: Option<ContextParamTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListShape {
    pub member: ListMember,
    #[serde(default)]
    pub traits: ListTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListTraits {
    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#externalDocumentation", default)]
    pub external_documentation: Option<IndexMap<String, String>>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
    #[serde(rename = "smithy.api#sparse", default)]
    pub sparse: Option<SparseTrait>,
    #[serde(rename = "smithy.api#uniqueItems", default)]
    pub unique_items: Option<UniqueItemsTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListMember {
    pub target: ShapeId,
    #[serde(default)]
    pub traits: ListMemberTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListMemberTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#tags", default)]
    pub tags: Vec<String>,
    #[serde(rename = "smithy.api#xmlName", default)]
    pub xml_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapShape {
    pub key: MapShapeRef,
    pub value: MapShapeRef,
    #[serde(default)]
    pub traits: MapTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
    #[serde(rename = "smithy.api#sparse", default)]
    pub sparse: Option<SparseTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapShapeRef {
    pub target: ShapeId,
    #[serde(default)]
    pub traits: MapShapeTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapShapeTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#pattern", default)]
    pub pattern: Option<String>,
    #[serde(rename = "smithy.api#tags", default)]
    pub tags: Vec<String>,
    #[serde(rename = "smithy.api#xmlName", default)]
    pub xml_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnionShape {
    pub members: IndexMap<String, UnionMember>,
    #[serde(default)]
    pub traits: UnionTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnionTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
    #[serde(rename = "smithy.api#streaming", default)]
    pub streaming: Option<StreamingTrait>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnionMember {
    pub target: ShapeId,
    #[serde(default)]
    pub traits: UnionMemberTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnionMemberTraits {
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#range", default)]
    pub range: Option<RangeTrait>,
    #[serde(rename = "smithy.api#tags", default)]
    pub tags: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnumShape {
    pub members: IndexMap<String, EnumMember>,
    #[serde(default)]
    pub traits: EnumTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnumTraits {
    #[serde(rename = "smithy.api#default", default)]
    pub default: Option<String>,
    #[serde(rename = "smithy.api#deprecated", default)]
    pub deprecated: Option<DeprecatedTrait>,
    #[serde(rename = "smithy.api#documentation", default)]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#sensitive", default)]
    pub sensitive: Option<SensitiveTrait>,
    #[serde(rename = "smithy.api#length", default)]
    pub length: Option<LengthTrait>,
    #[serde(rename = "smithy.api#pattern", default)]
    pub pattern: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnumMember {
    pub target: ShapeId,
    #[serde(default)]
    pub traits: EnumMemberTraits,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnumMemberTraits {
    #[serde(rename = "smithy.api#enumValue")]
    pub enum_value: String,
}
