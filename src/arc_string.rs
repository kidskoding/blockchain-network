use std::sync::Arc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct ArcString(Arc<String>);
impl Serialize for ArcString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
impl<'de> Deserialize<'de> for ArcString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(ArcString(Arc::new(s)))
    }
}
impl From<Arc<String>> for ArcString {
    fn from(arc_string: Arc<String>) -> Self {
        ArcString(arc_string)
    }
}
impl From<ArcString> for Arc<String> {
    fn from(arc_string: ArcString) -> Self {
        arc_string.0
    }
}
impl PartialEq for ArcString {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || *self.0 == *other.0
    }
}