// ArcScript parser skeleton

use crate::ast::{
    BinaryOp, Expr, FuncDecl, Literal, Param, Program, Stmt, UnaryOp,
};

use std::fmt;
use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl ParseError {
    pub fn new(line: usize, column: usize, message: String) -> Self {
        Self { line, column, message }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    pub errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next_token();
        Self { lexer, current, errors: Vec::new() }
    }

    pub fn error(&self, msg: &str) -> ParseError {
        ParseError::new(self.current.line, self.current.column, msg.to_string())
    }

    fn synchronize(&mut self) {
        // Advance until likely statement boundary to continue parsing after an error
        self.advance();
        while self.current.kind != TokenKind::Eof {
            match self.current.kind {
                TokenKind::Semicolon => { self.advance(); return; }
                TokenKind::KwFunc | TokenKind::KwVar | TokenKind::KwIf | TokenKind::KwWhile | TokenKind::KwReturn => return,
                _ => { self.advance(); }
            }
        }
    }

    fn consume(&mut self, kind: TokenKind, msg: &str) -> Result<(), ParseError> {
        if self.current.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("expected {:?}: {}", kind, msg)))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut body = Vec::new();
        while self.current.kind != TokenKind::Eof {
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(e) => {
                    self.errors.push(e);
                    self.synchronize();
                }
            }
        }
        if self.errors.is_empty() {
            Ok(Program { body })
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn consume(&mut self, kind: TokenKind, msg: &str) {
        if self.current.kind == kind {
            self.advance();
        } else {
            panic!("Parse error: expected {:?}, got {:?}: {}", kind, self.current.kind, msg);
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.current.kind {
            TokenKind::KwVar => self.parse_var_decl(),
            TokenKind::KwIf => self.parse_if_stmt(),
            TokenKind::KwWhile => self.parse_while_stmt(),
            TokenKind::KwReturn => self.parse_return_stmt(),
            TokenKind::KwFunc => {
                let func = self.parse_func_decl()?;
                Ok(Stmt::FuncDecl(func))
            }
            TokenKind::KwObject => {
                let obj = self.parse_object_decl()?;
                Ok(Stmt::ObjectDecl(obj))
            }
            TokenKind::LBrace => {
                let block = self.parse_block()?;
                Ok(Stmt::Block(block))
            }
            _ => {
                let expr = self.parse_expression()?;
                if self.current.kind == TokenKind::Semicolon {
                    self.advance();
                }
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'var'
        let name = if let TokenKind::Identifier = self.current.kind {
            let n = self.current.lexeme.clone();
            self.advance();
            n
        } else {
            return Err(self.error("expected identifier after 'var'"));
        };

        if self.current.kind == TokenKind::Colon {
            // skip optional type annotation for now
            self.advance();
            if let TokenKind::Identifier = self.current.kind {
                self.advance();
            }
        }

        self.consume(TokenKind::Equal, "expected '=' in var declaration")?;
        let init = self.parse_expression()?;
        if self.current.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::VarDecl { name, init })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        // assumes current token is '{'
        self.consume(TokenKind::LBrace, "expected '{' to start block")?;
        let mut stmts = Vec::new();
        while self.current.kind != TokenKind::RBrace && self.current.kind != TokenKind::Eof {
            match self.parse_statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => { self.errors.push(e); self.synchronize(); }
            }
        }
        self.consume(TokenKind::RBrace, "expected '}' to close block")?;
        Ok(stmts)
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenKind::KwIf, "expected 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenKind::KwThen, "expected 'then' after if condition")?;
        let then_block = Stmt::Block(self.parse_block()?);

        let mut elifs = Vec::new();
        while self.current.kind == TokenKind::KwElif {
            self.advance();
            let cond = self.parse_expression()?;
            self.consume(TokenKind::KwThen, "expected 'then' after elif condition")?;
            let block = Stmt::Block(self.parse_block()?);
            elifs.push((cond, block));
        }

        let else_branch = if self.current.kind == TokenKind::KwElse {
            self.advance();
            let block = Stmt::Block(self.parse_block()?);
            Some(Box::new(block))
        } else {
            None
        };

        self.consume(TokenKind::KwEnd, "expected 'end' after if statement")?;

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_block),
            elif_branches: elifs,
            else_branch,
        })
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenKind::KwWhile, "expected 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenKind::KwDo, "expected 'do' after while condition")?;
        let body = Stmt::Block(self.parse_block()?);
        self.consume(TokenKind::KwEnd, "expected 'end' after while statement")?;
        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenKind::KwReturn, "expected 'return'")?;
        let expr = if self.current.kind != TokenKind::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };
        if self.current.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Return(expr))
    }

    fn parse_func_decl(&mut self) -> Result<FuncDecl, ParseError> {
        self.consume(TokenKind::KwFunc, "expected 'func'")?;
        let name = if let TokenKind::Identifier = self.current.kind {
            let n = self.current.lexeme.clone();
            self.advance();
            n
        } else {
            return Err(self.error("expected function name after 'func'"));
        };

        self.consume(TokenKind::LParen, "expected '(' after function name")?;
        let mut params = Vec::new();
        if self.current.kind != TokenKind::RParen {
            loop {
                let param_name = if let TokenKind::Identifier = self.current.kind {
                    let n = self.current.lexeme.clone();
                    self.advance();
                    n
                } else {
                    return Err(self.error("expected parameter name"));
                };
                params.push(Param { name: param_name });
                if self.current.kind == TokenKind::Comma {
                    self.advance();
                    continue;
                }
                break;
            }
        }
        self.consume(TokenKind::RParen, "expected ')' after parameter list")?;

        // Optional ':' return type is currently ignored
        if self.current.kind == TokenKind::Colon {
            self.advance();
            if let TokenKind::Identifier = self.current.kind {
                self.advance();
            }
        }

        self.consume(TokenKind::Colon, "expected ':' before function body")?;
        let body_block = Stmt::Block(self.parse_block()?);
        self.consume(TokenKind::KwEnd, "expected 'end' after function body")?;

        Ok(FuncDecl {
            name,
            params,
            body: body_block,
        })
    }

    fn parse_object_decl(&mut self) -> Result<crate::ast::ObjectDecl, ParseError> {
        use crate::ast::{ObjectDecl, ObjectMember, EventDecl};
        
        self.consume(TokenKind::KwObject, "expected 'object'")?;
        let name = if let TokenKind::Identifier = self.current.kind {
            let n = self.current.lexeme.clone();
            self.advance();
            n
        } else {
            return Err(self.error("expected object name after 'object'"));
        };

        self.consume(TokenKind::Colon, "expected ':' after object name")?;
        self.consume(TokenKind::LBrace, "expected '{' to start object body")?;

        let mut members = Vec::new();
        while self.current.kind != TokenKind::RBrace && self.current.kind != TokenKind::Eof {
            match self.current.kind {
                TokenKind::KwVar => {
                    let var_stmt = self.parse_var_decl()?;
                    members.push(ObjectMember::Var(var_stmt));
                }
                TokenKind::KwFunc => {
                    let func = self.parse_func_decl()?;
                    members.push(ObjectMember::Method(func));
                }
                TokenKind::KwOn => {
                    let event = self.parse_event_decl()?;
                    members.push(ObjectMember::Event(event));
                }
                _ => {
                    return Err(self.error("expected 'var', 'func', or 'on' in object body"));
                }
            }
        }

        self.consume(TokenKind::RBrace, "expected '}' to close object body")?;
        self.consume(TokenKind::KwEnd, "expected 'end' after object")?;

        Ok(ObjectDecl { name, members })
    }

    fn parse_event_decl(&mut self) -> Result<crate::ast::EventDecl, ParseError> {
        use crate::ast::EventDecl;
        
        self.consume(TokenKind::KwOn, "expected 'on'")?;
        let name = if let TokenKind::Identifier = self.current.kind {
            let n = self.current.lexeme.clone();
            self.advance();
            n
        } else {
            return Err(self.error("expected event name after 'on'"));
        };

        self.consume(TokenKind::LParen, "expected '(' after event name")?;
        let mut params = Vec::new();
        if self.current.kind != TokenKind::RParen {
            loop {
                let param_name = if let TokenKind::Identifier = self.current.kind {
                    let n = self.current.lexeme.clone();
                    self.advance();
                    n
                } else {
                    return Err(self.error("expected parameter name"));
                };
                params.push(Param { name: param_name });
                if self.current.kind == TokenKind::Comma {
                    self.advance();
                    continue;
                }
                break;
            }
        }
        self.consume(TokenKind::RParen, "expected ')' after parameter list")?;
        self.consume(TokenKind::Colon, "expected ':' before event body")?;
        let body = Stmt::Block(self.parse_block()?);
        self.consume(TokenKind::KwEnd, "expected 'end' after event body")?;

        Ok(EventDecl { name, params, body })
    }

    fn parse_table_literal(&mut self) -> Result<Vec<crate::ast::TableField>, ParseError> {
        use crate::ast::TableField;
        
        self.consume(TokenKind::LBrace, "expected '{'")?;
        let mut fields = Vec::new();

        while self.current.kind != TokenKind::RBrace && self.current.kind != TokenKind::Eof {
            // Check if key:value or just value
            if self.current.kind == TokenKind::Identifier {
                let key_or_val = self.current.lexeme.clone();
                self.advance();
                if self.current.kind == TokenKind::Colon {
                    // key: value
                    self.advance();
                    let value = self.parse_expression()?;
                    fields.push(TableField::KeyValue { key: key_or_val, value });
                } else {
                    // just identifier as value - backtrack by creating Ident expr
                    let value = Expr::Ident(key_or_val);
                    fields.push(TableField::Value(value));
                }
            } else {
                // arbitrary expression as value
                let value = self.parse_expression()?;
                fields.push(TableField::Value(value));
            }

            if self.current.kind == TokenKind::Comma {
                self.advance();
            } else {
                break;
            }
        }

        self.consume(TokenKind::RBrace, "expected '}' to close table literal")?;
        Ok(fields)
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_and()?;
        while self.current.kind == TokenKind::KwOr {
            self.advance();
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;
        while self.current.kind == TokenKind::KwAnd {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;
        loop {
            let op = match self.current.kind {
                TokenKind::EqualEqual => Some(BinaryOp::Equal),
                TokenKind::BangEqual => Some(BinaryOp::NotEqual),
                _ => None,
            };
            if let Some(bin_op) = op {
                self.advance();
                let right = self.parse_comparison()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;
        loop {
            let op = match self.current.kind {
                TokenKind::Less => Some(BinaryOp::Less),
                TokenKind::LessEqual => Some(BinaryOp::LessEqual),
                TokenKind::Greater => Some(BinaryOp::Greater),
                TokenKind::GreaterEqual => Some(BinaryOp::GreaterEqual),
                _ => None,
            };
            if let Some(bin_op) = op {
                self.advance();
                let right = self.parse_term()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;
        loop {
            let op = match self.current.kind {
                TokenKind::Plus => Some(BinaryOp::Add),
                TokenKind::Minus => Some(BinaryOp::Sub),
                _ => None,
            };
            if let Some(bin_op) = op {
                self.advance();
                let right = self.parse_factor()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;
        loop {
            let op = match self.current.kind {
                TokenKind::Star => Some(BinaryOp::Mul),
                TokenKind::Slash => Some(BinaryOp::Div),
                _ => None,
            };
            if let Some(bin_op) = op {
                self.advance();
                let right = self.parse_unary()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.current.kind {
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(expr),
                })
            }
            TokenKind::KwNot => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match &self.current.kind {
            TokenKind::Int => {
                let value: i64 = self.current.lexeme.parse().unwrap_or(0);
                self.advance();
                Expr::Literal(Literal::Int(value))
            }
            TokenKind::Float => {
                let value: f64 = self.current.lexeme.parse().unwrap_or(0.0);
                self.advance();
                Expr::Literal(Literal::Float(value))
            }
            TokenKind::KwTrue => {
                self.advance();
                Expr::Literal(Literal::Bool(true))
            }
            TokenKind::KwFalse => {
                self.advance();
                Expr::Literal(Literal::Bool(false))
            }
            TokenKind::KwNil => {
                self.advance();
                Expr::Literal(Literal::Nil)
            }
            TokenKind::String => {
                let value = self.current.lexeme.clone();
                self.advance();
                Expr::Literal(Literal::String(value))
            }
            TokenKind::Identifier => {
                let name = self.current.lexeme.clone();
                self.advance();
                Expr::Ident(name)
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RParen, "expected ')' after expression")?;
                expr
            }
            TokenKind::LBrace => {
                // Table literal
                let fields = self.parse_table_literal()?;
                Expr::TableLiteral(fields)
            }
            _ => {
                return Err(self.error(&format!("unexpected token in primary expression: {:?}", self.current.kind)));
            }
        };

        // Handle postfix operations: calls, member access, indexing
        loop {
            match self.current.kind {
                TokenKind::LParen => {
                    // parse call arguments
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if self.current.kind != TokenKind::RParen {
                        loop {
                            let arg = self.parse_expression()?;
                            args.push(arg);
                            if self.current.kind == TokenKind::Comma {
                                self.advance();
                                continue;
                            }
                            break;
                        }
                    }
                    self.consume(TokenKind::RParen, "expected ')' after arguments")?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                TokenKind::Dot => {
                    // Member access: obj.field
                    self.advance();
                    let field = if let TokenKind::Identifier = self.current.kind {
                        let f = self.current.lexeme.clone();
                        self.advance();
                        f
                    } else {
                        return Err(self.error("expected field name after '.'"));
                    };
                    expr = Expr::Member {
                        object: Box::new(expr),
                        field,
                    };
                }
                TokenKind::LBracket => {
                    // Index access: obj[index]
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(TokenKind::RBracket, "expected ']' after index")?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_error_recovery() {
        let src = r#"
            var a = 1;
            func bad( {   // error
            var b = 2;
            var c = 3;
        "#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let res = parser.parse_program();
        assert!(res.is_err());
        let errs = res.err().unwrap();
        assert!(errs.len() >= 1);
    }
}
}
