use crate::ast::expr::{Expr, Literal, Value};
use crate::ast::stmt::{Block, ExprStmt, FunDecl, If, Print, Return, Stmt, Var, While};
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
            Fun => self.parse_fun_declaration(),
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

    fn parse_fun_declaration(&mut self) -> PResult<Stmt> {
        let start_span = self.bump().span;
        let id = self.expect_identifier()?;
        self.expect(
            TokenKind::LeftParen,
            "expect '(' after function identifier".into(),
        )?;

        let params = self.parse_params()?;

        self.expect(TokenKind::RightParen, "expect ')' after params".into())?;
        self.expect(
            TokenKind::LeftBrace,
            "expect '{' before function body".into(),
        )?;

        let mut body: Vec<Stmt> = Vec::new();
        while !self.peek().kind.match_kind(&TokenKind::RightBrace) && !self.is_at_end() {
            body.push(self.parse_declaration()?);
        }
        let end_span = self
            .expect(
                TokenKind::RightBrace,
                "expect '}' after function body".into(),
            )?
            .span;

        Ok(Stmt::FunDecl(FunDecl::new(
            start_span.union(&end_span),
            id,
            params,
            body,
        )))
    }

    fn parse_params(&mut self) -> PResult<Vec<Identifier>> {
        use TokenKind::*;

        let mut params = Vec::new();
        if !self.peek().kind.match_kind(&RightParen) {
            params.push(self.expect_identifier()?);
            while self.matches(&[Comma]).is_some() {
                params.push(self.expect_identifier()?);
            }
        }

        Ok(params)
    }

    fn parse_stmt(&mut self) -> PResult<Stmt> {
        use TokenKind::*;

        let token = self.peek();
        match token.kind {
            Print => self.parse_print(),
            LeftBrace => self.parse_block(),
            If => self.parse_if(),
            While => self.parse_while(),
            For => self.parse_for(),
            Return => self.parse_return(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_block(&mut self) -> PResult<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();
        let left_brace_span = self.bump().span;

        while !self.peek().kind.match_kind(&TokenKind::RightBrace) && !self.is_at_end() {
            stmts.push(self.parse_declaration()?);
        }

        let right_brace = self.expect(TokenKind::RightBrace, "did not find matching }".into())?;

        Ok(Stmt::Block(Block::new(
            left_brace_span.union(&right_brace.span),
            stmts,
        )))
    }

    fn parse_if(&mut self) -> PResult<Stmt> {
        let if_span = self.bump().span;

        self.expect(TokenKind::LeftParen, "expect '(' after 'if'".into())?;
        let condition = self.parse_expr()?;
        self.expect(
            TokenKind::RightParen,
            "expect ')' after 'if' condition".into(),
        )?;

        let then_stmt = self.parse_stmt()?;

        let (span, else_stmt) = if self.matches(&[TokenKind::Else]).is_some() {
            let else_stmt = self.parse_stmt()?;
            (if_span.union(&else_stmt.span()), Some(else_stmt))
        } else {
            (if_span.union(&then_stmt.span()), None)
        };

        Ok(Stmt::If(If::new(span, condition, then_stmt, else_stmt)))
    }

    fn parse_while(&mut self) -> PResult<Stmt> {
        let span_start = self.bump().span;
        self.expect(TokenKind::LeftParen, "expect '(' after 'while'".into())?;
        let cond = self.parse_expr()?;
        self.expect(
            TokenKind::RightParen,
            "expect ')' after 'while' condition".into(),
        )?;
        let stmt = self.parse_stmt()?;

        Ok(Stmt::While(While::new(
            span_start.union(&stmt.span()),
            cond,
            stmt,
        )))
    }

    fn parse_for(&mut self) -> PResult<Stmt> {
        let span_start = self.bump().span;
        self.expect(TokenKind::LeftParen, "expect '(' after 'for'".into())?;

        let initializer = self
            .match_and_or(&[TokenKind::Semicolon], None, |this| {
                match this.peek().kind {
                    TokenKind::Var => Some(this.parse_var_declaration()),
                    _ => Some(this.parse_expr_stmt()),
                }
            })
            .transpose()?;

        let condition = self
            .peek_and_or(&[TokenKind::Semicolon], None, |this| {
                Some(this.parse_expr())
            })
            .transpose()?
            .unwrap_or_else(|| Expr::Literal(Literal::new(self.peek().span, Value::Boolean(true))));

        self.expect(
            TokenKind::Semicolon,
            "expect ';' after loop condition".into(),
        )?;

        let increment = self
            .peek_and_or(&[TokenKind::RightParen], None, |this| {
                Some(this.parse_expr())
            })
            .transpose()?;

        self.expect(
            TokenKind::RightParen,
            "expect ')' after 'for' increment".into(),
        )?;

        let mut stmt = self.parse_stmt()?;
        let span = span_start.union(&stmt.span());

        if let Some(increment) = increment {
            stmt = Stmt::Block(Block::new(
                span,
                vec![stmt, Stmt::Expr(ExprStmt::new(increment.span(), increment))],
            ))
        }

        stmt = Stmt::While(While::new(span, condition, stmt));

        if let Some(initializer) = initializer {
            stmt = Stmt::Block(Block::new(span, vec![initializer, stmt]));
        }

        Ok(stmt)
    }

    fn parse_print(&mut self) -> PResult<Stmt> {
        let start = self.bump().span;

        let expr = self.parse_expr()?;
        let semicolon = self.expect(TokenKind::Semicolon, TERMINATOR.into())?;
        let span = start.union(&semicolon.span);

        Ok(Stmt::Print(Print::new(span, expr)))
    }

    fn parse_expr_stmt(&mut self) -> PResult<Stmt> {
        let expr = self.parse_expr()?;
        let semicolon = self.expect_semicolon()?;
        let span = expr.span().union(&semicolon.span);

        Ok(Stmt::Expr(ExprStmt::new(span, expr)))
    }

    fn parse_return(&mut self) -> PResult<Stmt> {
        let mut return_span = self.bump().span;
        let expr = if let Some(semicolon) = self.matches(&[TokenKind::Semicolon]) {
            let expr = Expr::Literal(Literal::new(return_span, Value::Nil));
            return_span = return_span.union(&semicolon.span);
            expr
        } else {
            let expr = self.parse_expr()?;
            let semicolon = self.expect_semicolon()?;
            return_span = return_span.union(&semicolon.span);
            expr
        };

        Ok(Stmt::Return(Return::new(return_span, expr)))
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
    use crate::ast::expr;
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

    #[test]
    fn parse_block_stmt() {
        let source = "{ a; { b; }}";
        let expected = Stmt::Block(Block::new(
            Span::new(0, 12),
            vec![
                Stmt::Expr(ExprStmt::new(
                    Span::new(2, 4),
                    Expr::Var(expr::Var::new(
                        Span::new(2, 3),
                        Identifier::new(Span::new(2, 3), "a", 0),
                    )),
                )),
                Stmt::Block(Block::new(
                    Span::new(5, 11),
                    vec![Stmt::Expr(ExprStmt::new(
                        Span::new(7, 9),
                        Expr::Var(expr::Var::new(
                            Span::new(7, 8),
                            Identifier::new(Span::new(7, 8), "b", 1),
                        )),
                    ))],
                )),
            ],
        ));

        let mut parser = Parser::new(source);
        let stmt = parser.parse_declaration().unwrap();

        assert_eq!(expected, stmt);
    }

    #[test]
    fn parse_if_stmt() {
        let source = "if (true) if (true) print 5; else print 6;";
        let expected = Stmt::If(If::new(
            Span::new(0, 42),
            Expr::Literal(Literal::new(Span::new(4, 8), Value::Boolean(true))),
            Stmt::If(If::new(
                Span::new(10, 42),
                Expr::Literal(Literal::new(Span::new(14, 18), Value::Boolean(true))),
                Stmt::Print(Print::new(
                    Span::new(20, 28),
                    Expr::Literal(Literal::new(Span::new(26, 27), Value::Number(5.0))),
                )),
                Some(Stmt::Print(Print::new(
                    Span::new(34, 42),
                    Expr::Literal(Literal::new(Span::new(40, 41), Value::Number(6.0))),
                ))),
            )),
            None as Option<Stmt>,
        ));

        let mut parser = Parser::new(source);
        let stmt = parser.parse_declaration().unwrap();

        assert_eq!(expected, stmt);
    }

    #[test]
    fn parse_while() {
        let source = "while (true) print 1;";
        let expected = Stmt::While(While::new(
            Span::new(0, 21),
            Expr::Literal(Literal::new(Span::new(7, 11), Value::Boolean(true))),
            Stmt::Print(Print::new(
                Span::new(13, 21),
                Expr::Literal(Literal::new(Span::new(19, 20), Value::Number(1.0))),
            )),
        ));

        let mut parser = Parser::new(source);
        let stmt = parser.parse_declaration().unwrap();

        assert_eq!(expected, stmt);
    }

    #[test]
    fn parse_return() {
        let source = "return 5;";
        let expected = Stmt::Return(Return::new(
            Span::new(0, 9),
            Expr::Literal(Literal::new(Span::new(7, 8), Value::Number(5.0))),
        ));

        let mut parser = Parser::new(source);
        let stmt = parser.parse_declaration().unwrap();

        assert_eq!(expected, stmt);
    }
}
