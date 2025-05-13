use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct InputReference<'a> {
    pub id: &'a str,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<&'a str>,
}

impl<'a> InputReference<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            type_field: None,
        }
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("item_reference");
        self
    }
}
