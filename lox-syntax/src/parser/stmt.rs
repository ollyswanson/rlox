use crate::ast::expr::{Expr, Literal, Value};
use crate::ast::stmt::{ExprStmt, Print, Stmt, Var};
use crate::ast::Identifier;
use crate::parser::error::{PResult, ParseError};
use crate::parser::Parser;
use crate::token::{Token, TokenKind};

static TERMINATOR: &str = "missing semicolon ';'";

impl<'a> Parser<'a> {
    // Would it be possible to switch to something that uses precedence or state instead of lots
    // of recursion?
    pub fn parse_declaration(&mut self) -> PResult<Stmt> {
        use TokenKind::*;

        match self.peek().kind {
            Var => self.parse_var_declaration(),
            _ => self.parse_stmt(),
        }
    }

    fn parse_var_declaration(&mut self) -> PResult<Stmt> {
        let var_token = self.bump();
        let start_span = var_token.span;
        let identifier = self.expect_identifier()?;

        let expr = if self.matches(&[TokenKind::Equal]).is_some() {
            self.parse_expr()?
        } else {
            Expr::Literal(Literal::new(identifier.span, Value::Nil))
        };

        let semicolon = self.expect_semicolon()?;

        Ok(Stmt::Var(Var::new(
            start_span.union(&semicolon.span),
            identifier,
            expr,
        )))
    }

    fn parse_stmt(&mut self) -> PResult<Stmt> {
        use TokenKind::*;

        let token = self.peek();
        match token.kind {
            Print => self.print(),
            _ => self.expr_stmt(),
        }
    }

    fn print(&mut self) -> PResult<Stmt> {
        let start = self.bump().span;

        let expr = self.parse_expr()?;
        let semicolon = self.expect(TokenKind::Semicolon, TERMINATOR.into())?;
        let span = start.union(&semicolon.span);

        Ok(Stmt::Print(Print::new(span, expr)))
    }

    fn expr_stmt(&mut self) -> PResult<Stmt> {
        let expr = self.parse_expr()?;
        let semicolon = self.expect_semicolon()?;
        let span = expr.span().union(&semicolon.span);

        Ok(Stmt::Expr(ExprStmt::new(span, expr)))
    }

    fn expect_semicolon(&mut self) -> PResult<&Token> {
        self.expect(TokenKind::Semicolon, TERMINATOR.into())
    }

    fn expect_identifier(&mut self) -> PResult<Identifier> {
        let id = self.increment();
        let token = self.bump();

        match &token.kind {
            TokenKind::Identifier(i) => Ok(Identifier::new(token.span, i, id)),
            _ => Err(ParseError::UnexpectedToken {
                message: format!("Expected identifier found {}", token.kind).into(),
                span: token.span,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expr::{Expr, Literal, Value};
    use crate::span::Span;

    use super::*;

    #[test]
    fn parse_print_stmt() {
        let source = "print 5;";
        let expected = Stmt::Print(Print::new(
            Span::new(0, 8),
            Expr::Literal(Literal::new(Span::new(6, 7), Value::Number(5.0))),
        ));

        let mut parser = Parser::new(source);
        let stmt = parser.parse_declaration().unwrap();

        assert_eq!(expected, stmt);
    }

    #[test]
    fn error_if_missing_semicolon() {
        let source = "5";

        let mut parser = Parser::new(source);
        let error = parser.parse_stmt();
        assert!(error.is_err());
    }

    #[test]
    fn parse_var_declarations() {
        let source = "var a = 5;";

        let expected = Stmt::Var(Var::new(
            Span::new(0, 10),
            Identifier::new(Span::new(4, 5), "a", 0),
            Expr::Literal(Literal::new(Span::new(8, 9), Value::Number(5.0))),
        ));

        let mut parser = Parser::new(source);
        let stmt = parser.parse_declaration().unwrap();

        assert_eq!(expected, stmt);
    }
}
