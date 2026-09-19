/*
Source provenance attached to canonical EDA objects.

Adapters use this metadata to preserve the relationship between normalized
objects and their native source files without leaking native syntax into models.
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceFormat {
    KiCad,
    Eagle,
    Altium,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    pub format: SourceFormat,
    pub file_path: String,
    pub native_id: Option<String>,
}

impl SourceRef {
    pub fn new(format: SourceFormat, file_path: impl Into<String>) -> Self {
        Self {
            format,
            file_path: file_path.into(),
            native_id: None,
        }
    }
}
