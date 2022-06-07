use crate::ast::expr::{Binary, Expr, Grouping, Literal, UnOp, Unary};
use crate::ast::util::{AssocOp, Fixity};
use crate::parser::error::{PResult, ParseError};
use crate::parser::Parser;
use crate::token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> PResult<Expr> {
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
                _ => todo!(),
            };
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> PResult<Expr> {
        use super::TokenKind::*;

        let token = self.peek();
        let span = token.span;

        match token.kind {
            String(_) | Number(_) | Nil | True | False => {
                Ok(Expr::Literal(Literal::from_token(self.bump()).unwrap()))
            }
            Minus | Bang => self.parse_unary(),
            LeftParen => self.parse_grouping(),
            Error(error) => Err(ParseError::ScanError { error, span }),
            _ => Err(ParseError::UnexpectedToken {
                span,
                message: format!("Unexpected token {}", token.kind).into(),
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
}

#[cfg(test)]
mod tests {
    use crate::ast::expr::{BinOp, Value};
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
}
