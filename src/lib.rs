mod parser;

type BoxedRegex = Box<Regex>;

#[derive(Debug, PartialEq)]
pub enum Regex {
    Character(char),
    Concatenation(BoxedRegex, BoxedRegex),
    Alternation(BoxedRegex, BoxedRegex),
    Repetition(BoxedRegex, Quantifier),
}

impl Regex {
    // fn from(input: &str) -> Option<Regex> {
    //     parser::parse_regex(input)
    // }
}

#[derive(Debug, PartialEq)]
pub struct Quantifier(QuantifierType, QuantifierModifier);

#[derive(Debug, PartialEq)]
pub enum QuantifierModifier {
    Greedy,
    Nongreedy,
}

#[derive(Debug, PartialEq)]
pub enum QuantifierType {
    ZeroOrOne,
    OneOrMore,
    ZeroOrMore,
}

pub fn character(c: char) -> BoxedRegex {
    Box::new(Regex::Character(c))
}

pub fn alternation(e1: BoxedRegex, e2: BoxedRegex) -> BoxedRegex {
    Box::new(Regex::Alternation(e1, e2))
}

pub fn concatenation(e1: BoxedRegex, e2: BoxedRegex) -> BoxedRegex {
    Box::new(Regex::Concatenation(e1, e2))
}

pub fn repetition(e: BoxedRegex, q: Quantifier) -> BoxedRegex {
    Box::new(Regex::Repetition(e, q))
}
