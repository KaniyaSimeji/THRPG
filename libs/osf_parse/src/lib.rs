use combine::parser::{
    char::{letter, space},
    Parser,
};
use combine::stream::Stream;
use combine::ParseError;
use combine::{between, many, many1, satisfy, skip_many, skip_many1, token};
use std::collections::HashMap;

/// osf contents
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Osf {
    counter: u32,
    contents: HashMap<String, HashMap<String, String>>,
}

impl Osf {
    // main parse
    pub fn osf(text: &str) -> anyhow::Result<Self> {
        let result = Self::parse().parse(text)?;
        Ok(result.0)
    }

    fn property<I>() -> impl Parser<I, Output = (String, String)>
    where
        I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        (
            many1(satisfy(|c| c != ':' && c != '[' && c != ']' && c != ';')),
            token(':'),
            many1(satisfy(|c| c != '\n' && c != ';' && c != '#')),
        )
            .map(|(key, _, value)| (key, value))
            .message("while parsing property")
    }

    fn whilespace<I>() -> impl Parser<I>
    where
        I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        let comment = (token('#'), skip_many(satisfy(|c| c != '\n'))).map(|_| ());
        skip_many(skip_many1(space()).or(comment))
    }

    fn properties<I>() -> impl Parser<I, Output = HashMap<String, String>>
    where
        I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        many(Self::property().skip(Self::whilespace()))
    }

    fn section<I>() -> impl Parser<I, Output = (String, HashMap<String, String>)>
    where
        I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        (
            between(token('['), token(']'), many(letter())),
            Self::whilespace(),
            Self::properties(),
        )
            .map(|(name, _, properties)| (name, properties))
            .message("while parsing section")
    }

    fn parse<I>() -> impl Parser<I, Output = Self>
    where
        I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        (
            Self::whilespace(),
            Self::properties(),
            many(Self::section()),
        )
            .map(|(_, _, section)| Self {
                counter: 0,
                contents: section,
            })
    }
}
