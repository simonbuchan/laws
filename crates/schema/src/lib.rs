use indexmap::IndexMap;

pub use endpoint_rules::*;
pub use shape_id::*;
pub use shapes::*;
pub use traits::*;

mod endpoint_rules;
mod shape_id;
mod shapes;
mod traits;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub smithy: SmithyVersion,
    #[serde(default)]
    pub metadata: Metadata,
    pub shapes: IndexMap<ShapeId, Shape>,
}

#[derive(Debug, serde::Deserialize)]
pub enum SmithyVersion {
    #[serde(rename = "2.0")]
    _2_0,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub suppressions: Vec<MetadataSuppression>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MetadataSuppression {
    pub id: String,
    pub namespace: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShapeRef {
    pub target: ShapeId,
}
