use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::common::ObjectId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeKind {
    Added,
    Removed,
    Modified,
    Unchanged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatchMethod {
    ObjectId,
    Reference,
    Name,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldChange {
    pub field: String,
    pub before: Value,
    pub after: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectChange {
    pub object_type: String,
    pub display_name: Option<String>,
    pub before_id: Option<ObjectId>,
    pub after_id: Option<ObjectId>,
    pub kind: ChangeKind,
    pub match_method: Option<MatchMethod>,
    pub fields: Vec<FieldChange>,
}

impl ObjectChange {
    pub fn added(
        object_type: impl Into<String>,
        display_name: Option<String>,
        after_id: ObjectId,
    ) -> Self {
        Self {
            object_type: object_type.into(),
            display_name,
            before_id: None,
            after_id: Some(after_id),
            kind: ChangeKind::Added,
            match_method: None,
            fields: Vec::new(),
        }
    }

    pub fn removed(
        object_type: impl Into<String>,
        display_name: Option<String>,
        before_id: ObjectId,
    ) -> Self {
        Self {
            object_type: object_type.into(),
            display_name,
            before_id: Some(before_id),
            after_id: None,
            kind: ChangeKind::Removed,
            match_method: None,
            fields: Vec::new(),
        }
    }

    pub fn compared(
        object_type: impl Into<String>,
        display_name: Option<String>,
        before_id: ObjectId,
        after_id: ObjectId,
        match_method: MatchMethod,
        fields: Vec<FieldChange>,
    ) -> Self {
        Self {
            object_type: object_type.into(),
            display_name,
            before_id: Some(before_id),
            after_id: Some(after_id),
            kind: if fields.is_empty() {
                ChangeKind::Unchanged
            } else {
                ChangeKind::Modified
            },
            match_method: Some(match_method),
            fields,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffSummary {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
    pub unchanged: usize,
}

impl DiffSummary {
    pub fn from_changes(changes: &[ObjectChange]) -> Self {
        let mut summary = Self::default();
        for change in changes {
            match change.kind {
                ChangeKind::Added => summary.added += 1,
                ChangeKind::Removed => summary.removed += 1,
                ChangeKind::Modified => summary.modified += 1,
                ChangeKind::Unchanged => summary.unchanged += 1,
            }
        }
        summary
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiffReport {
    pub summary: DiffSummary,
    pub changes: Vec<ObjectChange>,
}

impl DiffReport {
    pub fn new(changes: Vec<ObjectChange>) -> Self {
        let summary = DiffSummary::from_changes(&changes);
        Self { summary, changes }
    }
}
