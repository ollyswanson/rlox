use crate::ast::expr::{
    Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Set, UnOp, Unary, Var,
};
use crate::ast::util::{AssocOp, Fixity};
use crate::ast::Identifier;
use crate::parser::error::{PResult, ParseError};
use crate::parser::Parser;
use crate::span::Span;
use crate::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self) -> PResult<Expr> {
        self.parse_assoc_op_with_prec(0)
    }

    // Parse 15 / 3 / 5
    // start: prec = 0
    // lhs = 15
    // op = / ; next_prec = 7 fixity = 1 ; 7 > 0 continue;
    // recurse: prec = 8
    //   lhs = 3
    //   op = / ; next_prec = 7 ; 7 < 8 break ;  return 3
    // lhs = (/ 15 3)
    // op = / ; next_prec = 7 fixity = 1 ; continue
    // recurse: prec = 8
    //   lhs = 5
    //   op = None ; next_prec = 7 ; 7 < 8 break ; return 5
    // lhs = (/ (/ 15 3) 5)
    // op = None :: break
    // return lhs
    fn parse_assoc_op_with_prec(&mut self, prec: u8) -> PResult<Expr> {
        let mut lhs = self.parse_prefix()?;

        while let Some(op) = AssocOp::from_token(self.peek()) {
            let next_precedence = op.precedence();

            if next_precedence < prec {
                break;
            }

            self.bump();

            let fixity_adjustment = match op.fixity() {
                Fixity::Right => 0,
                Fixity::Left => 1,
            };

            let rhs = self.parse_assoc_op_with_prec(next_precedence + fixity_adjustment)?;

            let span = lhs.span().union(&rhs.span());

            lhs = match op {
                AssocOp::Equal
                | AssocOp::NotEqual
                | AssocOp::Greater
                | AssocOp::GreaterEqual
                | AssocOp::Less
                | AssocOp::LessEqual
                | AssocOp::Add
                | AssocOp::Subtract
                | AssocOp::Multiply
                | AssocOp::Divide => {
                    Expr::Binary(Binary::new(span, op.to_bin_op().unwrap(), lhs, rhs))
                }
                AssocOp::Assign => match lhs {
                    Expr::Var(var) => {
                        Expr::Assign(Assign::new(var.span.union(&rhs.span()), var.id, rhs))
                    }
                    Expr::Get(get) => Expr::Set(Set::new(span, get.object, get.property, rhs)),
                    _ => {
                        return Err(ParseError::InvalidAssignment {
                            span: lhs.span(),
                            message: format!("can't assign to {}", lhs).into(),
                        });
                    }
                },
                AssocOp::And | AssocOp::Or => {
                    Expr::Logical(Logical::new(span, op.to_logical_op().unwrap(), lhs, rhs))
                }
            };
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> PResult<Expr> {
        use super::TokenKind as T;

        let token = self.peek();
        let span = token.span;

        match token.kind {
            T::String(_) | T::Number(_) | T::Nil | T::True | T::False => {
                Ok(Expr::Literal(Literal::from_token(self.bump()).unwrap()))
            }
            T::Identifier(ref name) => {
                let name = name.clone();
                self.bump();

                let expr = Expr::Var(Var::new(
                    span,
                    Identifier::new(span, name, self.increment()),
                ));

                self.parse_call_or_get(expr)
            }
            T::Minus | T::Bang => self.parse_unary(),
            T::LeftParen => self.parse_grouping(),
            T::Error(error) => Err(ParseError::ScanError { error, span }),
            _ => Err(ParseError::UnexpectedToken {
                span,
                message: format!("Unexpected token {}", token.kind).into(),
                kind: token.kind.clone(),
            }),
        }
    }

    fn parse_unary(&mut self) -> PResult<Expr> {
        let op = self.bump();
        let op_span = op.span;
        let op = UnOp::from_token(op).unwrap();

        let expr = self.parse_prefix()?;

        Ok(Expr::Unary(Unary::new(
            op_span.union(&expr.span()),
            op,
            expr,
        )))
    }

    fn parse_grouping(&mut self) -> PResult<Expr> {
        let left_paren_span = self.bump().span;
        let expr = self.parse_expr()?;
        let right_paren = self.expect(
            TokenKind::RightParen,
            format!(
                "Expected {} found {}",
                TokenKind::RightParen,
                self.peek().kind
            )
            .into(),
        )?;

        let span = left_paren_span.union(&right_paren.span);

        Ok(Expr::Grouping(Grouping::new(span, expr)))
    }

    fn parse_call_or_get(&mut self, mut expr: Expr) -> PResult<Expr> {
        use TokenKind as T;
        loop {
            match self.peek().kind {
                T::LeftParen => {
                    let (right_span, args) = self.parse_arguments()?;
                    expr = Expr::Call(Call::new(expr.span().union(&right_span), expr, args));
                }
                T::Dot => {
                    // consume dot
                    self.bump();
                    let property = self.expect_identifier()?;
                    expr = Expr::Get(Get::new(expr.span().union(&property.span), expr, property))
                }
                _ => {
                    break;
                }
            }
        }
        Ok(expr)
    }

    fn parse_arguments(&mut self) -> PResult<(Span, Vec<Expr>)> {
        use TokenKind::*;

        // consume left paren
        self.bump();

        let mut args = Vec::new();
        if !self.peek().kind.match_kind(&RightParen) {
            args.push(self.parse_expr()?);
            while self.matches(&[Comma]).is_some() {
                args.push(self.parse_expr()?);
            }
        }
        let right_paren_span = self
            .expect(RightParen, "expect ')' after arguments".into())?
            .span;

        Ok((right_paren_span, args))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expr::{BinOp, Logical, LogicalOp, Value, Var};
    use crate::span::Span;

    use super::*;

    #[test]
    fn parse_left_associative_expressions() {
        let source = "15 / 5 / 3";
        let expected = Expr::Binary(Binary::new(
            Span::new(0, 10),
            BinOp::Divide,
            Expr::Binary(Binary::new(
                Span::new(0, 6),
                BinOp::Divide,
                Expr::Literal(Literal::new(Span::new(0, 2), Value::Number(15.0))),
                Expr::Literal(Literal::new(Span::new(5, 6), Value::Number(5.0))),
            )),
            Expr::Literal(Literal::new(Span::new(9, 10), Value::Number(3.0))),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parses_left_associate_mixed_expressions() {
        let source = "15 + 5 / 3";
        let expected = Expr::Binary(Binary::new(
            Span::new(0, 10),
            BinOp::Add,
            Expr::Literal(Literal::new(Span::new(0, 2), Value::Number(15.0))),
            Expr::Binary(Binary::new(
                Span::new(5, 10),
                BinOp::Divide,
                Expr::Literal(Literal::new(Span::new(5, 6), Value::Number(5.0))),
                Expr::Literal(Literal::new(Span::new(9, 10), Value::Number(3.0))),
            )),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parses_grouped_expressions() {
        let source = "(15 + 5) / 3";
        let expected = Expr::Binary(Binary::new(
            Span::new(0, 12),
            BinOp::Divide,
            Expr::Grouping(Grouping::new(
                Span::new(0, 8),
                Expr::Binary(Binary::new(
                    Span::new(1, 7),
                    BinOp::Add,
                    Expr::Literal(Literal::new(Span::new(1, 3), Value::Number(15.0))),
                    Expr::Literal(Literal::new(Span::new(6, 7), Value::Number(5.0))),
                )),
            )),
            Expr::Literal(Literal::new(Span::new(11, 12), Value::Number(3.0))),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parses_unary_expressions() {
        let source = "15 + -5";
        let expected = Expr::Binary(Binary::new(
            Span::new(0, 7),
            BinOp::Add,
            Expr::Literal(Literal::new(Span::new(0, 2), Value::Number(15.0))),
            Expr::Unary(Unary::new(
                Span::new(5, 7),
                UnOp::Minus,
                Expr::Literal(Literal::new(Span::new(6, 7), Value::Number(5.0))),
            )),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parses_var_expressions() {
        let source = "a_variable";
        let expected = Expr::Var(Var::new(
            Span::new(0, 10),
            Identifier::new(Span::new(0, 10), "a_variable", 0),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parses_assignment_right_associative() {
        let source = "a = b = 5";
        let expected = Expr::Assign(Assign::new(
            Span::new(0, 9),
            Identifier::new(Span::new(0, 1), "a", 0),
            Expr::Assign(Assign::new(
                Span::new(4, 9),
                Identifier::new(Span::new(4, 5), "b", 1),
                Expr::Literal(Literal::new(Span::new(8, 9), Value::Number(5.0))),
            )),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parses_logical_expressions() {
        let source = "a or b and c and d";
        let expected = Expr::Logical(Logical::new(
            Span::new(0, 18),
            LogicalOp::Or,
            Expr::Var(Var::new(
                Span::new(0, 1),
                Identifier::new(Span::new(0, 1), "a", 0),
            )),
            Expr::Logical(Logical::new(
                Span::new(5, 18),
                LogicalOp::And,
                Expr::Logical(Logical::new(
                    Span::new(5, 12),
                    LogicalOp::And,
                    Expr::Var(Var::new(
                        Span::new(5, 6),
                        Identifier::new(Span::new(5, 6), "b", 1),
                    )),
                    Expr::Var(Var::new(
                        Span::new(11, 12),
                        Identifier::new(Span::new(11, 12), "c", 2),
                    )),
                )),
                Expr::Var(Var::new(
                    Span::new(17, 18),
                    Identifier::new(Span::new(17, 18), "d", 3),
                )),
            )),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parses_calls() {
        let source = "foo()(1)";
        let expected = Expr::Call(Call::new(
            Span::new(0, 8),
            Expr::Call(Call::new(
                Span::new(0, 5),
                Expr::Var(Var::new(
                    Span::new(0, 3),
                    Identifier::new(Span::new(0, 3), "foo", 0),
                )),
                Vec::new(),
            )),
            vec![Expr::Literal(Literal::new(
                Span::new(6, 7),
                Value::Number(1.0),
            ))],
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parse_get() {
        let source = "foo().bar.baz";
        let expected = Expr::Get(Get::new(
            Span::new(0, 13),
            Expr::Get(Get::new(
                Span::new(0, 9),
                Expr::Call(Call::new(
                    Span::new(0, 5),
                    Expr::Var(Var::new(
                        Span::new(0, 3),
                        Identifier::new(Span::new(0, 3), "foo", 0),
                    )),
                    vec![],
                )),
                Identifier::new(Span::new(6, 9), "bar", 1),
            )),
            Identifier::new(Span::new(10, 13), "baz", 2),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }

    #[test]
    fn parse_set() {
        let source = "foo.bar.baz = 1";
        let expected = Expr::Set(Set::new(
            Span::new(0, 15),
            Expr::Get(Get::new(
                Span::new(0, 7),
                Expr::Var(Var::new(
                    Span::new(0, 3),
                    Identifier::new(Span::new(0, 3), "foo", 0),
                )),
                Identifier::new(Span::new(4, 7), "bar", 1),
            )),
            Identifier::new(Span::new(8, 11), "baz", 2),
            Expr::Literal(Literal::new(Span::new(14, 15), Value::Number(1.0))),
        ));

        let mut parser = Parser::new(source);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(expected, expr);
    }
}
