use indexmap::IndexMap;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndpointRuleSet {
    pub parameters: IndexMap<String, EndpointRuleSetParam>,
    pub rules: Vec<EndpointRuleItem>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointRuleSetParam {
    #[serde(default)]
    pub built_in: Option<EndpointRuleSetParamBuiltIn>,
    pub required: bool,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    pub documentation: String,
    #[serde(rename = "type")]
    pub type_: EndpointRuleSetParamType,
}

#[derive(Debug, serde::Deserialize)]
pub enum EndpointRuleSetParamBuiltIn {
    #[serde(rename = "AWS::Region")]
    Region,
    #[serde(rename = "AWS::UseDualStack")]
    UseDualStack,
    #[serde(rename = "AWS::UseFIPS")]
    UseFIPS,
    #[serde(rename = "AWS::S3::Accelerate")]
    S3Accelerate,
    #[serde(rename = "AWS::S3::DisableMultiRegionAccessPoints")]
    S3DisableMultiRegionAccessPoints,
    #[serde(rename = "AWS::S3::ForcePathStyle")]
    S3ForcePathStyle,
    #[serde(rename = "AWS::S3::UseArnRegion")]
    S3UseArnRegion,
    #[serde(rename = "AWS::S3::UseGlobalEndpoint")]
    S3UseGlobalEndpoint,
    #[serde(rename = "AWS::S3Control::UseArnRegion")]
    S3ControlUseArnRegion,
    #[serde(rename = "AWS::STS::UseGlobalEndpoint")]
    StsUseGlobalEndpoint,
    #[serde(rename = "SDK::Endpoint")]
    SdkEndpoint,
}

#[derive(Debug, serde::Deserialize)]
pub enum EndpointRuleSetParamType {
    Boolean,
    String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointRuleItem {
    pub conditions: Vec<EndpointRuleCondition>,
    #[serde(flatten)]
    pub rule: EndpointRule,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndpointRuleCondition {
    #[serde(default)]
    pub assign: Option<String>,
    #[serde(flatten)]
    pub function: EndpointRuleConditionFn,
}

#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "fn",
    content = "argv",
    rename_all = "camelCase",
    deny_unknown_fields
)]
pub enum EndpointRuleConditionFn {
    Not(#[serde(deserialize_with = "single_tuple::deserialize")] EndpointRuleExpr),
    BooleanEquals(EndpointRuleExpr, EndpointRuleExpr),
    StringEquals(EndpointRuleExpr, EndpointRuleExpr),
    GetAttr(EndpointRuleExpr, String),
    IsSet(#[serde(deserialize_with = "single_tuple::deserialize")] EndpointRuleExpr),
    ParseURL(#[serde(deserialize_with = "single_tuple::deserialize")] EndpointRuleExpr),
    IsValidHostLabel(EndpointRuleExpr, bool),
    Substring(EndpointRuleExpr, usize, usize, bool),
    UriEncode(#[serde(deserialize_with = "single_tuple::deserialize")] EndpointRuleExpr),
    #[serde(rename = "aws.partition")]
    AwsPartition(#[serde(deserialize_with = "single_tuple::deserialize")] EndpointRuleExpr),
    #[serde(rename = "aws.parseArn")]
    AwsParseArn(#[serde(deserialize_with = "single_tuple::deserialize")] EndpointRuleExpr),
    #[serde(rename = "aws.isVirtualHostableS3Bucket")]
    AwsIsVirtualHostableS3Bucket(EndpointRuleExpr, bool),
}

mod single_tuple {
    pub fn deserialize<'de, D, V>(deserializer: D) -> Result<V, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: serde::Deserialize<'de>,
    {
        struct Visitor<V>(std::marker::PhantomData<V>);
        let v = deserializer.deserialize_tuple(1, Visitor::<V>(std::marker::PhantomData))?;
        return Ok(v);

        impl<'de, V> serde::de::Visitor<'de> for Visitor<V>
        where
            V: serde::Deserialize<'de>,
        {
            type Value = V;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a single element tuple")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let Some(v) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(0, &self));
                };
                if seq.next_element::<()>()?.is_some() {
                    return Err(serde::de::Error::invalid_length(2, &self));
                }
                Ok(v)
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum EndpointRuleExpr {
    Condition(Box<EndpointRuleCondition>),
    Reference {
        #[serde(rename = "ref")]
        name: String,
    },
    String(String),
    Boolean(bool),
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum EndpointRule {
    Tree { rules: Vec<EndpointRuleItem> },
    Error { error: String },
    Endpoint { endpoint: Endpoint },
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Endpoint {
    pub url: EndpointRuleExpr,
    pub properties: EndpointPropertyMap,
    pub headers: IndexMap<String, Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointPropertyMap {
    #[serde(default)]
    pub backend: Option<String>,
    #[serde(default)]
    pub auth_schemes: Vec<EndpointAuthScheme>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "name", rename_all = "kebab-case", deny_unknown_fields)]
pub enum EndpointAuthScheme {
    #[serde(rename_all = "camelCase")]
    Sigv4 {
        signing_name: String,
        signing_region: String,
        #[serde(default)]
        disable_double_encoding: bool,
    },
    #[serde(rename_all = "camelCase")]
    Sigv4a {
        signing_name: String,
        signing_region_set: Vec<String>,
        #[serde(default)]
        disable_double_encoding: bool,
    },
    #[serde(rename = "sigv4-s3express", rename_all = "camelCase")]
    Sigv4S3Express {
        signing_name: String,
        signing_region: String,
        #[serde(default)]
        disable_double_encoding: bool,
    },
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    macro_rules! assert_matches {
        () => {};
        ($value: expr => $pat: pat $(, $($tt: tt)*)? ) => {
            match $value {
                $pat => {
                    $(assert_matches!( $($tt)*);)?
                },
                _ => panic!(
                    "assertion failed: `{:?}` does not match `{}`",
                    $value,
                    stringify!($pat)
                ),
            }
        };
    }

    #[test]
    fn rule_conditions_boolean_equals() -> serde_json::Result<()> {
        let value = serde_json::from_value::<EndpointRuleCondition>(json!({
            "fn": "booleanEquals",
            "argv": [
                false,
                { "fn": "isSet", "argv": [{ "ref": "AWS::Region" }] },
            ],
        }))?;
        assert_matches!(
            value => EndpointRuleCondition {
                function: EndpointRuleConditionFn::BooleanEquals(
                    EndpointRuleExpr::Boolean(false),
                    EndpointRuleExpr::Condition(condition),
                ),
                assign: None,
            },
            *condition => EndpointRuleCondition {
                function: EndpointRuleConditionFn::IsSet(EndpointRuleExpr::Reference { name }),
                assign: None,
            },
            name.as_str() => "AWS::Region",
        );
        Ok(())
    }

    #[test]
    fn rule_tree() -> serde_json::Result<()> {
        let value = serde_json::from_value::<EndpointRuleItem>(json!({
            "conditions": [
                {
                    "fn": "booleanEquals",
                    "argv": [
                        false,
                        { "fn": "isSet", "argv": [{ "ref": "AWS::Region" }] },
                    ],
                },
            ],
            "rules": [],
            "type": "tree",
        }))?;

        assert_matches!(
            value => EndpointRuleItem { conditions, rule: EndpointRule::Tree { rules } },
            conditions.as_slice() => [EndpointRuleCondition {
                function: EndpointRuleConditionFn::BooleanEquals(
                    EndpointRuleExpr::Boolean(false),
                    EndpointRuleExpr::Condition(condition),
                ),
                assign: None,
            }],
            &**condition => EndpointRuleCondition {
                function: EndpointRuleConditionFn::IsSet(EndpointRuleExpr::Reference { name }),
                assign: None,
            },
            name.as_str() => "AWS::Region",
            rules.as_slice() => [],
        );

        Ok(())
    }

    #[test]
    fn rule_endpoint() -> serde_json::Result<()> {
        let value = serde_json::from_value::<EndpointRuleItem>(json!({
            "conditions": [
                {
                    "fn": "booleanEquals",
                    "argv": [
                        false,
                        { "fn": "isSet", "argv": [{ "ref": "AWS::Region" }] },
                    ],
                },
            ],
            "endpoint": {
                "url": { "ref": "https://{service}.{region}.amazonaws.com" },
                "properties": {
                    "service": "service",
                    "region": "region",
                },
                "headers": {
                    "X-Foo": "bar",
                },
            },
            "type": "endpoint",
        }))?;

        assert_matches!(
            value => EndpointRuleItem {
                conditions,
                rule: EndpointRule::Endpoint { endpoint },
            },
            conditions.as_slice() => [EndpointRuleCondition {
                function: EndpointRuleConditionFn::BooleanEquals(
                    EndpointRuleExpr::Boolean(false),
                    EndpointRuleExpr::Condition(condition),
                ),
                assign: None,
            }],
            &**condition => EndpointRuleCondition {
                function: EndpointRuleConditionFn::IsSet(EndpointRuleExpr::Reference { name: condition_name }),
                assign: None,
            },
            condition_name.as_str() => "AWS::Region",
            endpoint => Endpoint { url: EndpointRuleExpr::Reference { name: endpoint_name }, .. },
            endpoint_name.as_str() => "https://{service}.{region}.amazonaws.com",
        );

        Ok(())
    }

    #[test]
    fn expr_is_set() -> serde_json::Result<()> {
        let value = serde_json::from_value::<EndpointRuleExpr>(json!({
            "fn": "isSet",
            "argv": [{ "ref": "AWS::Region" }],
        }))?;

        assert_matches!(
            value => EndpointRuleExpr::Condition(condition),
            *condition => EndpointRuleCondition {
                function: EndpointRuleConditionFn::IsSet(EndpointRuleExpr::Reference { name }),
                assign: None,
            },
            name.as_str() => "AWS::Region",
        );

        Ok(())
    }

    #[test]
    fn expr_get_attr() -> serde_json::Result<()> {
        let value = serde_json::from_value::<EndpointRuleExpr>(json!({
            "fn": "getAttr",
            "argv": [
                { "ref": "AWS::Region" },
                "name",
            ],
        }))?;

        assert_matches!(
            value => EndpointRuleExpr::Condition(condition),
            *condition => EndpointRuleCondition {
                function: EndpointRuleConditionFn::GetAttr(
                    EndpointRuleExpr::Reference { name },
                    attr,
                ),
                assign: None,
            },
            name.as_str() => "AWS::Region",
            attr.as_str() => "name",
        );

        Ok(())
    }
}
