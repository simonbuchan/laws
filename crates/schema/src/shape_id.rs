use std::fmt;

#[derive(Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShapeId {
    pub namespace: String,
    pub name: String,
}

impl<'de> serde::Deserialize<'de> for ShapeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let shape_id = String::deserialize(deserializer)?;
        if shape_id.contains('$') {
            // used for method references
            return Err(serde::de::Error::custom(format!(
                "Unexpected '$' {shape_id}"
            )));
        }
        let (namespace, name) = shape_id
            .split_once('#')
            .ok_or_else(|| serde::de::Error::custom(format!("Missing '#': {shape_id:?}")))?;
        let namespace = namespace.to_string();
        let name = name.to_string();
        Ok(ShapeId { namespace, name })
    }
}

impl fmt::Debug for ShapeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.namespace, self.name)
    }
}

impl fmt::Display for ShapeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.namespace, self.name)
    }
}
    
