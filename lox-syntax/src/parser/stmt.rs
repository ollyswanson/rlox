use crate::ast::stmt::{ExprStmt, Print, Stmt};
use crate::parser::error::PResult;
use crate::token::{Token, TokenKind};
use crate::Parser;

static TERMINATOR: &str = "missing semicolon ';'";

impl<'a> Parser<'a> {
    pub fn parse_stmt(&mut self) -> PResult<Stmt> {
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
        let stmt = parser.parse_stmt().unwrap();

        assert_eq!(expected, stmt);
    }

    #[test]
    fn error_if_missing_semicolon() {
        let source = "5";

        let mut parser = Parser::new(source);
        let error = parser.parse_stmt();
        assert!(error.is_err());
    }
}
