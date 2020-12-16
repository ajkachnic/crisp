use std::iter::{Peekable};
use std::str::Chars;

use super::token::{Token, lookup_keyword, Operator, StringObject, CommentType};

pub struct Lexer<'a> {
  input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
  pub fn new(input: &str) -> Lexer {
    Lexer {
      input: input.chars().peekable()
    }
  }

  pub fn read(&mut self) -> Option<char> {
    self.input.next()
  }
  pub fn peek(&mut self) -> Option<&char> {
    self.input.peek()
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    match self.read() {
      Some(ch) => {
        match ch {
          '(' => Token::LeftParen,
          ')' => Token::RightParen,
          '[' => Token::LeftBrace,
          ']' => Token::RightBrace,
          '{' => Token::LeftBracket,
          '}' => Token::RightBracket,
          ',' => Token::Comma,
          ';' => self.next_token(),
          '~' => Token::Operator(Operator::BitwiseNot),
          '^' => Token::Operator(Operator::BitwiseXor),
          '?' => {
            match self.peek() {
              Some('?') => {
                self.read();
                Token::Operator(Operator::NullishCoalescing)
              },
              _ => Token::Operator(Operator::Ternary)
            }
          },
          '&' => {
            match self.peek() {
              Some('&') => {
                self.read();
                Token::Operator(Operator::LogicalAnd)
              },
              Some('=') => {
                self.read();
                Token::Operator(Operator::BitwiseAndAssign)
              },
              _ => Token::Operator(Operator::BitwiseAnd)
            }
          },
          '|' => {
            match self.peek() {
              Some('|') => {
                self.read();
                Token::Operator(Operator::LogicalOr)
              },
              Some('=') => {
                self.read();
                Token::Operator(Operator::BitwiseOrAssign)
              },
              _ => Token::Operator(Operator::BitwiseOr)
            }
          },
          '+' => {
            match self.peek() {
              Some('=') => {
                self.read();
                Token::Operator(Operator::PlusAssign)
              },
              Some('+') => {
                self.read();
                Token::Operator(Operator::PlusPlus)
              }
              _ => Token::Operator(Operator::Plus)
            }
          }, 
          '-' => {
            match self.peek() {
              Some('=') => {
                self.read();
                Token::Operator(Operator::MinusAssign)
              },
              Some('+') => {
                self.read();
                Token::Operator(Operator::MinusMinus)
              }
              _ => Token::Operator(Operator::Minus)
            }
          },
          '*' => {
            match self.peek() {
              Some('=') => {
                self.read();
                Token::Operator(Operator::AsteriskAssign)
              },
              Some('*') => {
                self.read();
                match self.peek() {
                  Some('=') => {
                    self.read();
                    Token::Operator(Operator::ExponentAssign)
                  },
                  _ => Token::Operator(Operator::Exponent)
                }
              }
              _ => Token::Operator(Operator::Asterisk)
            }
          },
          '/' => {
            match self.peek() {
              Some('/') => {
                  self.read_comment(CommentType::SingleLine);
                  
                  self.next_token()
              },
              Some('*') => {
                self.read();
                self.read_comment(CommentType::MultiLine);
                self.next_token()
              }
              Some('=') => {
                self.read();
                Token::Operator(Operator::SlashAssign)
              },
              _ => Token::Operator(Operator::Slash)
            }
          },
          '%' => {
            match self.peek() {
              Some('=') => {
                self.read();
                Token::Operator(Operator::ModuloAssign)
              },
              _ => Token::Operator(Operator::Modulo)
            }
          },
          '=' => {
            match self.peek() {
              Some('=') => {
                self.read();
                match self.peek() {
                  Some('=') => {
                    self.read();
                    Token::Operator(Operator::StrictEqual)
                  },
                  _ => Token::Operator(Operator::Equal)
                }
              },
              Some('>') => {
                self.read();
                Token::Operator(Operator::Arrow)
              },
              _ => Token::Operator(Operator::Assign)
            }
          },
          '!' => {
            match self.peek() {
              Some('=') => {
                self.read();
                match self.peek() {
                  Some('=') => {
                    self.read();
                    Token::Operator(Operator::StrictNotEqual)
                  },
                  _ => Token::Operator(Operator::NotEqual)
                }
              },
              _ => Token::Operator(Operator::Bang)
            }
          },
          '.' => {
            match self.peek() {
              Some('.') => {
                // TODO: fix double peek here, it results in .. being turned into .
                self.read();
                match self.peek() {
                  Some('.') => {
                    self.read();
                    Token::Operator(Operator::Spread)
                 },
                  _ => Token::Operator(Operator::Period)
                }
              },
              Some(&ch) => {
                if ch.is_numeric() {
                  let number = self.read_number(ch);
                  return Token::NUMBER(number)
                }
                Token::Operator(Operator::Period)
              },
              None => Token::Operator(Operator::Period)
            }
          },
          '>' => {
            match self.peek() {
              Some('>') => {
                self.read();
                match self.peek() {
                  Some('>') => {
                    self.read();
                    Token::Operator(Operator::UnsignedBitwiseRight)
                  },
                  _ => Token::Operator(Operator::BitwiseRight)
                }
              },
              Some('=') => {
                self.read();
                Token::Operator(Operator::GreaterThanEqual)
              }
              _ => Token::Operator(Operator::GreaterThan)
            }
          },
          '<' => {
            match self.peek() {
              Some('<') => {
                self.read();
                Token::Operator(Operator::BitwiseLeft)
              },
              Some('=') => {
                self.read();
                Token::Operator(Operator::LessThanEqual)
              }
              _ => Token::Operator(Operator::LessThan)
            }
          },
          '"' => {
            let string = self.read_string('"');
            Token::STRING(
              StringObject::new(string, '"')
            )
          },
          '\'' => {
            let string = self.read_string('\'');
            Token::STRING(
              StringObject::new(string, '\'')
            )
          },
          '`' => {
            let string = self.read_string('`');
            Token::STRING(
              StringObject::new(string, '`')
            )
          },
          ch => {
            if ch.is_numeric() {
              let number = self.read_number(ch);
              return Token::NUMBER(number);
            }
            let ident = self.read_indentifier(ch);
            if ident == String::new() {
              return Token::Illegal
            }
            lookup_keyword(ident)
          }
        }
      },
      None => {
        Token::EOF
      }
    }
  }

  fn peek_is_whitespace(&mut self) -> bool {
    match self.peek() {
      Some(&ch) => ch.is_whitespace(),
      None => false
    }
  }
  fn skip_whitespace(&mut self) {
    while self.peek_is_whitespace() {
      self.read();
    }
  }
  fn peek_is_char(&mut self) -> bool {
    match self.peek() {
      Some(&ch) => {
        return ch.is_alphabetic() || ch.is_digit(10) || ch == '_'
      },
      None => false
    }
  }

  fn read_indentifier(&mut self, ch: char) -> String {
    let mut ident = String::from(ch);
    while self.peek_is_char() {
      let ch = self.read().unwrap(); // The loop will break if the value is none, so unwrap is safe here
      ident.push(ch);
    }

    ident
  }
  
  fn peek_is_valid_char(&mut self, invalid: char) -> bool {
    match self.peek() {
      Some(&ch) => {
        return ch != invalid
      },
      None => false
    }
  }

  fn read_string(&mut self, ch: char) -> String {
    let end_char = ch;
    let mut string = String::new();
    while self.peek_is_valid_char(end_char)  {
      let ch = self.read().unwrap();
      string.push(ch);
    }
    self.read();
    string
  }

  fn peek_is_number(&mut self) -> bool {
    match self.peek() {
      Some(&ch) => ch.is_numeric() || ch == '.',
      None => false
    }
  }

  fn read_number(&mut self, ch: char) -> String {
    let mut number = String::from(ch);
    while self.peek_is_number() {
      let ch = self.read().unwrap();
      number.push(ch)
    }
    number
  }

  fn read_comment(&mut self, t: CommentType) {
    loop {
      let ch = self.peek();
      match t {
        CommentType::SingleLine => {
          if ch == Some(&'\n') || ch == None {
            break
          }
        },
        CommentType::MultiLine => {
          if ch == Some(&'*') || ch == None {
            self.read();
            let ch = self.peek();
            if ch == Some(&'/') {
              self.read();
              break
            }
          }
        }
      };
      self.read();
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;
  fn next(&mut self) -> Option<Token> {
    match self.next_token() {
      Token::EOF => None,
      value => Some(value)
    }
  }
}