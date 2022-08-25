mod parser;

type BoxedRegex = Box<Regex>;

/// Internal representation for a regular expression.
#[derive(Debug, PartialEq)]
pub enum Regex {
    Match(Match),
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
pub enum Match {
    Character(char),
    AnyCharacter,
}

impl From<Match> for Regex {
    fn from(m: Match) -> Self {
        Regex::Match(m)
    }
}

/// Quantifier for expressions.
/// The first component specifies the type of the quantifier (how many).
/// The second component specifies the modifier of the quantifier (greediness).
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
    Box::new(Match::Character(c).into())
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
