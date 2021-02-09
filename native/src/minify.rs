use super::lexer::Lexer;
use super::token::{Keyword, Operator, Token};

use std::fmt::Display;
use std::iter::Peekable;

pub struct Minifier<'a> {
    lex: Peekable<Lexer<'a>>,
}

impl<'a> Minifier<'a> {
    pub fn new(lex: Lexer) -> Minifier {
        Minifier {
            lex: lex.peekable(),
        }
    }

    fn handle_keyword<T>(value: T, keyword: &Keyword) -> String
    where
        T: Display,
    {
        match keyword {
            Keyword::IN => format!("{} ", value),
            Keyword::INSTANCEOF => format!("{} ", value),
            _ => format!("{};", value),
        }
    }

    pub fn generate_string(&mut self) -> String {
        let mut code = String::new();
        loop {
            let tok = self.lex.next();
            match tok {
                None => break,
                Some(tok) => match tok {
                    Token::LeftParen => code.push('('),
                    Token::RightParen => code.push_str(&self.build_rule_string(')')),
                    Token::LeftBrace => code.push('['),
                    Token::RightBrace => code.push_str(&self.build_rule_string(']')),
                    Token::LeftBracket => code.push('{'),
                    Token::RightBracket => code.push_str(&self.build_rule_string('}')),
                    Token::Comma => code.push(','),
                    Token::STRING(string) => match self.lex.peek() {
                        Some(Token::Ident(_)) => code.push_str(&format!("{};", string)),
                        Some(Token::Keyword(keyword)) => {
                            code.push_str(&Self::handle_keyword(string, keyword))
                        }
                        _ => code.push_str(&format!("{}", string)),
                    },
                    Token::NUMBER(number) => match self.lex.peek() {
                        Some(Token::Ident(_)) => {
                            code.push_str(&format!("{};", number));
                        }
                        Some(Token::Keyword(keyword)) => {
                            code.push_str(&Self::handle_keyword(number, keyword))
                        }
                        _ => code.push_str(&format!("{}", number)),
                    },

                    Token::Ident(value) => match self.lex.peek() {
                        Some(Token::Keyword(keyword)) => {
                            code.push_str(&Self::handle_keyword(value, keyword))
                        }
                        Some(Token::Ident(_)) => code.push_str(&format!("{};", value)),
                        _ => code.push_str(&value),
                    },
                    Token::Operator(op) => {
                        let code_value = match op {
                            Operator::Arrow => "=>",
                            Operator::Assign => "=",
                            Operator::Asterisk => "*",
                            Operator::AsteriskAssign => "*=",
                            Operator::Bang => "!",
                            Operator::BitwiseAnd => "&",
                            Operator::BitwiseAndAssign => "&=",
                            Operator::BitwiseLeft => "<<",
                            Operator::BitwiseNot => "~",
                            Operator::BitwiseOr => "|",
                            Operator::BitwiseOrAssign => "|=",
                            Operator::BitwiseRight => ">>",
                            Operator::BitwiseXor => "^",
                            Operator::Colon => ":",
                            Operator::Equal => "==",
                            Operator::Exponent => "**",
                            Operator::ExponentAssign => "**=",
                            Operator::GreaterThan => ">",
                            Operator::GreaterThanEqual => ">=",
                            Operator::LessThan => "<",
                            Operator::LessThanEqual => "<=",
                            Operator::LogicalAnd => "&&",
                            Operator::LogicalOr => "||",
                            Operator::Minus => "-",
                            Operator::MinusAssign => "-=",
                            Operator::MinusMinus => "--",
                            Operator::Modulo => "%",
                            Operator::ModuloAssign => "%=",
                            Operator::NotEqual => "!=",
                            Operator::NullishCoalescing => "??",
                            Operator::Period => ".",
                            Operator::Plus => "+",
                            Operator::PlusAssign => "+=",
                            Operator::PlusPlus => "++",
                            Operator::Slash => "/",
                            Operator::SlashAssign => "/=",
                            Operator::Spread => "...",
                            Operator::StrictEqual => "===",
                            Operator::StrictNotEqual => "!==",
                            Operator::Ternary => "?",
                            Operator::UnsignedBitwiseRight => ">>>",
                        };

                        code.push_str(code_value);
                    }
                    Token::Keyword(keyword) => match keyword {
                        Keyword::ASYNC => code.push_str("async "),
                        Keyword::AWAIT => code.push_str("await "),
                        Keyword::BREAK => code.push_str("break"),
                        Keyword::CASE => code.push_str("case "),
                        Keyword::CATCH => code.push_str("catch"),
                        Keyword::CLASS => code.push_str("class "),
                        Keyword::CONST => code.push_str("const "),
                        Keyword::CONTINUE => code.push_str("continue "),
                        Keyword::DEBUGGER => code.push_str("debugger "),
                        Keyword::DEFAULT => code.push_str("default "),
                        Keyword::DELETE => code.push_str("delete "),
                        Keyword::DO => code.push_str("do "),
                        Keyword::ELSE => code.push_str("else "),
                        Keyword::ENUM => code.push_str("enum "),
                        Keyword::EXPORT => code.push_str("export "),
                        Keyword::EXTENDS => code.push_str("extends "),
                        Keyword::FALSE => code.push_str("false"),
                        Keyword::FINALLY => code.push_str("finally "),
                        Keyword::FOR => code.push_str("for"),
                        Keyword::FUNCTION => code.push_str("function "),
                        Keyword::IF => code.push_str("if"),
                        Keyword::IMPLEMENTS => code.push_str("implements "),
                        Keyword::IMPORT => code.push_str("import "),
                        Keyword::IN => code.push_str("in "),
                        Keyword::INSTANCEOF => code.push_str("instanceof "),
                        Keyword::INTERFACE => code.push_str("interface "),
                        Keyword::LET => code.push_str("let "),
                        Keyword::NEW => code.push_str("new "),
                        Keyword::NULL => code.push_str("null"),
                        Keyword::PACKAGE => code.push_str("package"),
                        Keyword::PRIVATE => code.push_str("private "),
                        Keyword::PROTECTED => code.push_str("protected "),
                        Keyword::PUBLIC => code.push_str("public "),
                        Keyword::RETURN => code.push_str("return "),
                        Keyword::SUPER => code.push_str("super "),
                        Keyword::SWITCH => code.push_str("switch"),
                        Keyword::STATIC => code.push_str("static "),
                        Keyword::THIS => code.push_str("this"),
                        Keyword::THROW => code.push_str("throw "),
                        Keyword::TRY => code.push_str("try"),
                        Keyword::TRUE => code.push_str("true"),
                        Keyword::TYPEOF => code.push_str("typeof "),
                        Keyword::VAR => code.push_str("var "),
                        Keyword::VOID => code.push_str("void"),
                        Keyword::WHILE => code.push_str("while"),
                        Keyword::WITH => code.push_str("with"),
                        Keyword::YIELD => code.push_str("yield "),
                    },
                    _ => {}
                },
            }
        }
        code
    }
    fn build_rule_string(&mut self, ch: char) -> String {
        match self.lex.peek() {
            Some(Token::Ident(_)) => {
                if ch == '}' {
                    return String::from(ch);
                }
                format!("{};", ch)
            }
            Some(Token::Keyword(_)) => {
                if ch == '}' {
                    return format!("{}", ch);
                }
                format!("{};", ch)
            }
            _ => String::from(ch),
        }
    }
}
