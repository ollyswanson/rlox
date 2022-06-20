use std::collections::hash_map::Entry;
use std::collections::HashMap;

use lox_syntax::ast::stmt::Stmt;
use lox_syntax::Identifier;

use crate::resolver::error::ResolverError;
use crate::Interpreter;

mod error;
mod expr;
mod stmt;

#[derive(Debug)]
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, BindingState>>,
    // Errors from resolution step, we collect without short-circuiting in order to report all
    // all errors back to the user. The AST has already been created without errors so continuation
    // makes sense.
    diagnostics: Vec<ResolverError>,
}

#[derive(Debug)]
pub enum BindingState {
    Declared,
    Defined,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    pub fn diagnostics(&self) -> &[ResolverError] {
        &self.diagnostics
    }

    fn declare(&mut self, id: &Identifier) {
        if let Some(scope) = self.scopes.last_mut() {
            match scope.entry(id.name.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(BindingState::Declared);
                }
                Entry::Occupied(_) => self.error(ResolverError::AlreadyDeclared { span: id.span }),
            }
        }
    }

    fn define(&mut self, id: &Identifier) {
        if let Some(scope) = self.scopes.last_mut() {
            match scope.get_mut(&id.name) {
                Some(binding_state) => *binding_state = BindingState::Defined,
                None => self.error(ResolverError::Undeclared {
                    span: id.span,
                    message: format!("{} is undeclared", &id.name),
                }),
            }
        }
    }

    fn resolve_binding(&mut self, id: &Identifier) {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&id.name) {
                self.interpreter.resolve(id, depth);
                return;
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn scoped<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.begin_scope();
        f(self);
        self.end_scope();
    }

    fn error(&mut self, error: ResolverError) {
        self.diagnostics.push(error);
    }
}