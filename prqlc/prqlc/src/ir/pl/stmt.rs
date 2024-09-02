use enum_as_inner::EnumAsInner;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::pr::{Ident, QueryDef, Ty};
use crate::Span;

use super::{Expr, FuncCall};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Stmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
    #[serde(flatten)]
    pub kind: StmtKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, EnumAsInner, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum StmtKind {
    QueryDef(Box<QueryDef>),
    VarDef(VarDef),
    TypeDef(TypeDef),
    ModuleDef(ModuleDef),
    ImportDef(ImportDef),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VarDef {
    pub name: String,
    pub value: Option<Box<Expr>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ty: Option<Ty>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TypeDef {
    pub name: String,
    pub value: Option<Ty>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleDef {
    pub name: String,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ImportDef {
    pub alias: Option<String>,
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Annotation {
    pub expr: Box<Expr>,
}

impl Annotation {
    /// Utility to match function calls by name and unpack its arguments.
    pub fn as_func_call(&self, name: &str) -> Option<&FuncCall> {
        let call = self.expr.kind.as_func_call()?;

        let func_name = call.name.kind.as_ident()?;
        if func_name.len() != 1 || func_name.name != name {
            return None;
        }
        Some(call)
    }
}
