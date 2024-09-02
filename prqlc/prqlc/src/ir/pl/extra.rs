use enum_as_inner::EnumAsInner;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ir::generic::WindowKind;
use crate::ir::pl::{Expr, ExprKind, FuncCall, Ident, Range};
use crate::pr::Ty;

impl FuncCall {
    pub fn new_simple(name: Expr, args: Vec<Expr>) -> Self {
        FuncCall {
            name: Box::new(name),
            args,
            named_args: Default::default(),
        }
    }
}

/// An expression that may have already been converted to a type.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, EnumAsInner, JsonSchema)]
pub enum TyOrExpr {
    Ty(Ty),
    Expr(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct FuncMetadata {
    /// Name of the function. Used for user-facing messages only.
    pub name_hint: Option<Ident>,

    pub implicit_closure: Option<Box<ImplicitClosureConfig>>,
    pub coerce_tuple: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImplicitClosureConfig {
    pub param: u8,
    pub this: Option<u8>,
    pub that: Option<u8>,
}

impl FuncMetadata {
    pub(crate) fn as_debug_name(&self) -> &str {
        let ident = self.name_hint.as_ref();

        ident.map(|n| n.name.as_str()).unwrap_or("<anonymous>")
    }
}

pub type WindowFrame = crate::ir::generic::WindowFrame<Box<Expr>>;
pub type ColumnSort = crate::ir::generic::ColumnSort<Box<Expr>>;

/// FuncCall with better typing. Returns the modified table.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TransformCall {
    pub input: Box<Expr>,

    pub kind: Box<TransformKind>,

    /// Grouping of values in columns
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<Box<Expr>>,

    /// Windowing frame of columns
    #[serde(default, skip_serializing_if = "WindowFrame::is_default")]
    pub frame: WindowFrame,

    /// Windowing order of columns
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<ColumnSort>,
}

#[derive(
    Debug, PartialEq, Clone, Serialize, Deserialize, strum::AsRefStr, EnumAsInner, JsonSchema,
)]
pub enum TransformKind {
    Derive {
        assigns: Box<Expr>,
    },
    Select {
        assigns: Box<Expr>,
    },
    Filter {
        filter: Box<Expr>,
    },
    Aggregate {
        assigns: Box<Expr>,
    },
    Sort {
        by: Vec<ColumnSort>,
    },
    Take {
        range: Range,
    },
    Join {
        side: JoinSide,
        with: Box<Expr>,
        filter: Box<Expr>,
    },
    Group {
        by: Box<Expr>,
        pipeline: Box<Expr>,
    },
    Window {
        kind: WindowKind,
        range: Range,
        pipeline: Box<Expr>,
    },
    Append(Box<Expr>),
    Loop(Box<Expr>),
}

/// A reference to a table that is not in scope of this query.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum TableExternRef {
    /// Actual table in a database, that we can refer to by name in SQL
    LocalTable(Ident),

    /// Placeholder for a relation that will be provided later.
    /// This is very similar to relational s-strings and may not even be needed for now, so
    /// it's not documented anywhere. But it will be used in the future.
    Param(String),
    // TODO: add other sources such as files, URLs
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum JoinSide {
    Inner,
    Left,
    Right,
    Full,
}

impl Expr {
    pub fn new(kind: impl Into<ExprKind>) -> Self {
        Expr {
            id: None,
            kind: kind.into(),
            span: None,
            target_id: None,
            ty: None,
            needs_window: false,
            alias: None,
            flatten: false,
        }
    }
}
