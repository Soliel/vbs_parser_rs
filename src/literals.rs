use nom::bytes::complete::{tag_no_case, take_until, take_while};
use nom::character::complete::{char, digit0, digit1, multispace0};
use nom::character::is_digit;
use nom::combinator::map_res;
use nom::error::{Error, ParseError, ErrorKind};
use nom::{Err, Finish, IResult};
use nom::branch::alt;
use nom::sequence::{delimited, preceded};
use crate::tokens::{Token, TokenType};

type MapRes<'a> = Result<Token, Err<Error<&'a str>>>;


fn trim_leading_ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(
        multispace0,
        inner
    )
}

fn parse_i32<'a>(input: &str) -> IResult<&str, Token>
{
    let res: IResult<&str, &str> = trim_leading_ws(
        take_while(|c: char| c.is_digit(10))
    )(input);

    match res {
        Ok((remaining, result)) => {
            let int_result = i32::from_str_radix(result, 10);
            match int_result {
                Ok(i32_result) => Ok((remaining, Token::new(result, i32_result))),
                Err(_) => Err(Err::Error(Error::new(input, ErrorKind::Fail)))
            }
        }
        Err(_) => Err(Err::Error(Error::new(input, ErrorKind::Fail)))
    }
}

fn parse_string(input: &str) -> IResult<&str, Token> {
    map_res(
        delimited(char('\"'), take_until("\""), char('\"')),
        |result: &str| -> MapRes {
            Ok(Token::new(result, result))
        }
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, Token> {
    alt((
        map_res(
            tag_no_case("True"),
            |result: &str| -> MapRes {
                Ok(Token::new(result, true))
            }),
        map_res(
            tag_no_case("False"),
            |result: &str| -> MapRes {
                Ok(Token::new(result, false))
            }
        ))
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::literals::{parse_bool, parse_i32, parse_string};
    use crate::tokens::{Token, TokenType};

    #[test]
    fn int_test() {
        assert_eq!(parse_i32("123"), Ok(("", Token::new("123", 123))));
        assert_eq!(parse_i32(" 123 "), Ok((" ", Token::new("123", 123))));
        assert_eq!(parse_i32("12 3"), Ok((" 3", Token::new("12", 12))));
    }

    #[test]
    fn string_test() {
        assert_eq!(parse_string("\"123\""), Ok(("", Token::new("123", "123"))))
    }

    #[test]
    fn bool_test() {
        assert_eq!(parse_bool("true"), Ok(("", Token::new("true", true))))
    }
}