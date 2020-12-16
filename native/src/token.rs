use std::fmt;
#[derive(PartialEq, Debug)]
pub enum Token {
  STRING(StringObject),
  NUMBER(String),
  Illegal,
  Operator(Operator),
  Comma, // ,
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  
  Keyword(Keyword),
  Ident(String),
  EOF
}
#[derive(PartialEq, Debug)]
pub struct StringObject {
  pub value: String,
  pub t: StringType
}

impl StringObject {
  pub fn new(value: String, ch: char) -> StringObject {
    let t = match ch {
      '\'' => StringType::SingleQuote,
      '"' => StringType::DoubleQuote,
      '`' => StringType::Backtick,
      _ => StringType::SingleQuote
    };
    StringObject {
      value,
      t
    }
  }
}

impl fmt::Display for StringObject {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}{}", self.t, self.value, self.t)
  }
}

#[derive(PartialEq, Debug)]
pub enum StringType {
  SingleQuote,
  DoubleQuote,
  Backtick
}

impl fmt::Display for StringType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let ch = match *self {
      StringType::SingleQuote => '\'',
      StringType::DoubleQuote => '"',
      StringType::Backtick => '`'
    };
    write!(f, "{}", ch)
  }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
 // Tradition Operators
  Plus,       // +
  Minus,      // -
  Asterisk,   // *
  Slash,      // /
  Ternary,    // ?
  Spread,     // ...
  Modulo,     // %
  Exponent,   // **
  PlusPlus,   // ++
  MinusMinus, // --
  Period,     // .
  Bang,       // !
 
  Arrow,      // =>
  
  // Bitwise Operators
  BitwiseAnd, // &
  BitwiseOr,  // |
  BitwiseNot, // ~
  BitwiseXor, // ^
  BitwiseLeft,// <<
  BitwiseRight,// >>
  UnsignedBitwiseRight,// >>>,
  
  
  // Logical Operators
  LogicalOr,  // ||
  LogicalAnd, // &&
  NullishCoalescing, // ??
  
  // Assignment Operators
  Assign,     // =
  PlusAssign, // +=
  MinusAssign, // -=
  AsteriskAssign, // *=
  SlashAssign,  // /=
  ExponentAssign, // **=
  ModuloAssign, // %=
  BitwiseAndAssign, // &=
  BitwiseOrAssign, // |=

  
  // Comparison Operators
  GreaterThan,      // >
  LessThan,         // <
  GreaterThanEqual, // >=
  LessThanEqual,    // <=
  Equal,            // ==
  NotEqual,         // !=
  StrictEqual,      // ===
  StrictNotEqual,   // !==

}

#[derive(PartialEq, Debug)]
pub enum Keyword {
  ASYNC,
  AWAIT,
  BREAK,
  CASE,
  CATCH,
  CLASS,
  CONST,
  CONTINUE,
  DEBUGGER,
  DEFAULT,
  DELETE,
  DO,
  ELSE,
  ENUM,
  EXPORT,
  EXTENDS,
  FALSE,
  FINALLY,
  FOR,
  FUNCTION,
  IF,
  IMPLEMENTS,
  IMPORT,
  IN,
  INSTANCEOF,
  INTERFACE,
  LET,
  NEW,
  NULL,
  PACKAGE,
  PRIVATE,
  PROTECTED,
  PUBLIC,
  RETURN,
  SUPER,
  SWITCH,
  STATIC,
  THIS,
  THROW,
  TRY,
  TRUE,
  TYPEOF,
  VAR,
  VOID,
  WHILE,
  WITH,
  YIELD
}


pub fn lookup_keyword(word: String) -> Token {
  match word.as_str() {
    "async" => Token::Keyword(Keyword::ASYNC),
    "await" => Token::Keyword(Keyword::AWAIT),
    "break" => Token::Keyword(Keyword::BREAK),
    "case" => Token::Keyword(Keyword::CASE),
    "catch" => Token::Keyword(Keyword::CATCH),
    "class" => Token::Keyword(Keyword::CLASS),
    "const" => Token::Keyword(Keyword::CONST),
    "continue" => Token::Keyword(Keyword::CONTINUE),
    "debugger" => Token::Keyword(Keyword::DEBUGGER),
    "default" => Token::Keyword(Keyword::DEFAULT),
    "delete" => Token::Keyword(Keyword::DELETE),
    "do" => Token::Keyword(Keyword::DO),
    "else" => Token::Keyword(Keyword::ELSE),
    "enum" => Token::Keyword(Keyword::ENUM),
    "export" => Token::Keyword(Keyword::EXPORT),
    "extends" => Token::Keyword(Keyword::EXTENDS),
    "false" => Token::Keyword(Keyword::FALSE),
    "finally" => Token::Keyword(Keyword::FINALLY),
    "for" => Token::Keyword(Keyword::FOR),
    "function" => Token::Keyword(Keyword::FUNCTION),
    "if" => Token::Keyword(Keyword::IF),
    "implements" => Token::Keyword(Keyword::IMPLEMENTS),
    "import" => Token::Keyword(Keyword::IMPORT),
    "in" => Token::Keyword(Keyword::IN),
    "instanceof" => Token::Keyword(Keyword::INSTANCEOF),
    "interface" => Token::Keyword(Keyword::INTERFACE),
    "let" => Token::Keyword(Keyword::LET),
    "new" => Token::Keyword(Keyword::NEW),
    "null" => Token::Keyword(Keyword::NULL),
    "package" => Token::Keyword(Keyword::PACKAGE),
    "private" => Token::Keyword(Keyword::PRIVATE),
    "protected" => Token::Keyword(Keyword::PROTECTED),
    "public" => Token::Keyword(Keyword::PUBLIC),
    "return" => Token::Keyword(Keyword::RETURN),
    "super" => Token::Keyword(Keyword::SUPER),
    "switch" => Token::Keyword(Keyword::SWITCH),
    "static" => Token::Keyword(Keyword::STATIC),
    "this" => Token::Keyword(Keyword::THIS),
    "throw" => Token::Keyword(Keyword::THROW),
    "try" => Token::Keyword(Keyword::TRY),
    "true" => Token::Keyword(Keyword::TRUE),
    "typeof" => Token::Keyword(Keyword::TYPEOF),
    "var" => Token::Keyword(Keyword::VAR),
    "void" => Token::Keyword(Keyword::VOID),
    "while" => Token::Keyword(Keyword::WHILE),
    "with" => Token::Keyword(Keyword::WITH),
    "yield" => Token::Keyword(Keyword::YIELD),
    value => Token::Ident(value.to_string())
  }
}

pub enum CommentType {
  SingleLine,
  MultiLine
}