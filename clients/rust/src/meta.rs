//! Schema registries the generated client binds before issuing queries.

use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock, RwLock};

/// How a scalar field is compared in `where` / `having`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarFilter {
    /// Text.
    String,
    /// Integer or float.
    Number,
    /// Boolean.
    Boolean,
    /// Datetime.
    Datetime,
    /// JSON object without a declared shape.
    Json,
    /// Array.
    Array,
    /// Record id.
    Id,
}

impl ScalarFilter {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Datetime => "datetime",
            Self::Json => "json",
            Self::Array => "array",
            Self::Id => "id",
        }
    }
}

/// Graph direction of an `@relation` field relative to the parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelationDir {
    /// Parent is `in`; the far record is `out`.
    Out,
    /// Parent is `out`; the far record is `in`.
    In,
}

impl RelationDir {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Out => "out",
            Self::In => "in",
        }
    }
}

/// Select / where metadata for one field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldSelectMeta {
    /// Scalar column.
    Scalar {
        /// Comparison kind.
        filter: ScalarFilter,
    },
    /// Nested object.
    Object {
        /// Child fields.
        fields: BTreeMap<String, Self>,
    },
    /// Stored `@link`.
    Stored {
        /// Target table.
        target_table: String,
        /// List link.
        list: bool,
    },
    /// Computed `@link`.
    Computed {
        /// Target table.
        target_table: String,
        /// List link.
        list: bool,
    },
    /// `@relation` edge.
    Edge {
        /// Edge table.
        edge_table: String,
        /// Direction from the parent.
        dir: RelationDir,
        /// List relation.
        list: bool,
    },
}

impl FieldSelectMeta {
    /// Scalar filter metadata.
    #[must_use]
    pub fn scalar(filter: ScalarFilter) -> Self {
        Self::Scalar { filter }
    }

    pub(crate) fn list(&self) -> bool {
        match self {
            Self::Stored { list, .. } | Self::Computed { list, .. } | Self::Edge { list, .. } => {
                *list
            }
            Self::Scalar { .. } | Self::Object { .. } => false,
        }
    }

    pub(crate) const fn filter_kind(&self) -> Option<ScalarFilter> {
        match self {
            Self::Scalar { filter } => Some(*filter),
            _ => None,
        }
    }
}

/// Write metadata for one field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldWriteMeta {
    /// Plain column.
    Scalar,
    /// Stored `@link`.
    Stored {
        /// Target table.
        target_table: String,
        /// List link.
        list: bool,
        /// Field may be cleared.
        optional: bool,
        /// Unique scalar fields on the target, used by `where`.
        unique_fields: Vec<String>,
    },
    /// Computed `@link`.
    Computed {
        /// Target table.
        target_table: String,
        /// List link.
        list: bool,
        /// Stored field on the target that points back.
        back_link_field: String,
        /// Back-link may be cleared.
        back_link_optional: bool,
        /// Unique scalar fields on the target.
        unique_fields: Vec<String>,
    },
    /// `@relation` edge.
    Edge {
        /// Edge table.
        edge_table: String,
        /// Direction from the parent.
        dir: RelationDir,
        /// List relation.
        list: bool,
        /// `in` table.
        in_table: String,
        /// `out` table.
        out_table: String,
        /// Edge payload columns besides `in` / `out`.
        payload_fields: Vec<String>,
    },
}

/// `table -> field -> select meta`.
pub type SelectMeta = BTreeMap<String, BTreeMap<String, FieldSelectMeta>>;
/// `table -> field -> write meta`.
pub type WriteMeta = BTreeMap<String, BTreeMap<String, FieldWriteMeta>>;

static SELECT_META: LazyLock<RwLock<Arc<SelectMeta>>> =
    LazyLock::new(|| RwLock::new(Arc::new(BTreeMap::new())));
static WRITE_META: LazyLock<RwLock<Arc<WriteMeta>>> =
    LazyLock::new(|| RwLock::new(Arc::new(BTreeMap::new())));

fn read_arc<T>(lock: &RwLock<Arc<T>>) -> Arc<T> {
    lock.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

/// Install the registries emitted with a generated client.
pub fn bind_schema_meta(select_meta: SelectMeta, write_meta: WriteMeta) {
    *SELECT_META.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Arc::new(select_meta);
    *WRITE_META.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Arc::new(write_meta);
}

pub(crate) fn select_meta() -> Arc<SelectMeta> {
    read_arc(&SELECT_META)
}

pub(crate) fn write_meta() -> Arc<WriteMeta> {
    read_arc(&WRITE_META)
}
