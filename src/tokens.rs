#[derive(PartialEq, Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    IntegerLiteral(i32),
    BooleanLiteral(bool),
    StringLiteral(String),
    OpPlus,
    OpMinus,
    OpMul,
    OpDivide,
}

impl From<i32> for TokenType {
    fn from(val: i32) -> Self {
        TokenType::IntegerLiteral(val)
    }
}

impl From<bool> for TokenType {
    fn from(val: bool) -> Self {
        TokenType::BooleanLiteral(val)
    }
}

impl From<&str> for TokenType {
    fn from (val: &str) -> Self {
        TokenType::StringLiteral(val.to_owned())
    }
}

impl Token {
    pub fn new<F>(lexeme_new: &str, token_type_new: F) -> Token
        where F: Into<TokenType>
    {
        Token {
            lexeme: lexeme_new.to_owned(),
            token_type: token_type_new.into()
        }
    }
}