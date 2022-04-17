use sqlparser::ast::{Expr, Query, SetExpr, Statement, Value, Values};

use crate::error::{Result, SQLRiteError};

#[derive(Debug)]
pub struct DeleteQuery {
    pub table_name: String,
    pub selection: Option<Expr>,
}

impl DeleteQuery {
    pub fn new(statement: &Statement) -> Result<DeleteQuery> {
        #[allow(unused_assignments)]
        let mut tname: Option<String> = None;
        let mut tselection: Option<Expr> = None;

        match statement {
            Statement::Delete {
                table_name,
                selection,
            } => {
                tname = Some(table_name.to_string());
                tselection = selection.clone();
            }
            _ => return Err(SQLRiteError::Internal("Error parsing delete query".to_string())),
        }

        match tname {
            Some(t) => Ok(DeleteQuery {
                table_name: t,
                selection: tselection,
            }),
            None => Err(SQLRiteError::Internal("Error parsing delete query".to_string())),
        }
    }
}