use crate::{BoxedRegex, Quantifier, QuantifierModifier, QuantifierType, Regex};

type Result<'a, A> = std::result::Result<(A, &'a str), ParseError>;

#[derive(Debug, PartialEq)]
enum ParseError {
    EndOfInput,
    InvalidQuantifier,
    UnexpectedCharacter,
}

pub fn parse_regex(input: &str) -> Option<BoxedRegex> {
    match parse_repetition(input) {
        Ok((expr, "")) => Some(expr),
        _ => None,
    }
}

fn parse_expression(input: &str) -> Result<BoxedRegex> {
    parse_alternation(input)
}

fn parse_alternation(input: &str) -> Result<BoxedRegex> {
    let (e, input) = parse_concatenation(input)?;
    let mut expr = e;
    let mut input = input;
    while let Ok((_, next)) = expect_character(input, '|') {
        let (e, next) = parse_concatenation(next)?;
        expr = Box::new(Regex::Alternation(expr, e));
        input = next;
    }
    Ok((expr, input))
}

fn parse_concatenation(input: &str) -> Result<BoxedRegex> {
    let (e, input) = parse_repetition(input)?;
    let mut input = input;
    let mut expr = e;
    while let Ok((e, next)) = parse_repetition(input) {
        expr = Box::new(Regex::Concatenation(expr, e));
        input = next;
    }

    Ok((expr, input))
}

fn parse_repetition(input: &str) -> Result<BoxedRegex> {
    let mut input = input;
    let (e, input) = parse_primary(input)?;
    if let Ok((q, next)) = parse_quantifier(input) {
        return Ok((BoxedRegex::new(Regex::Repetition(e, q)), next));
    }
    Ok((e, input))
}

fn parse_primary(input: &str) -> Result<BoxedRegex> {
    parse_character(input).or_else(|_| parse_nested_regex(input))
}

fn parse_nested_regex(input: &str) -> Result<BoxedRegex> {
    let (_, input) = expect_character(input, '(')?;
    let (expr, input) = parse_expression(input)?;
    let (_, input) = expect_character(input, ')')?;
    Ok((expr, input))
}

fn parse_character(input: &str) -> Result<BoxedRegex> {
    if input.is_empty() {
        return Err(ParseError::EndOfInput);
    }

    let first = input.chars().next().unwrap();
    if first >= 'A' && first <= 'z' {
        Ok((Box::new(Regex::Character(first)), &input[1..]))
    } else {
        Err(ParseError::UnexpectedCharacter)
    }
}

fn expect_character(input: &str, c: char) -> Result<()> {
    if input.is_empty() {
        return Err(ParseError::EndOfInput);
    }

    let first = input.chars().next().unwrap();
    if first == c {
        Ok(((), &input[1..]))
    } else {
        Err(ParseError::UnexpectedCharacter)
    }
}

fn parse_quantifier(input: &str) -> Result<Quantifier> {
    if input.is_empty() {
        return Err(ParseError::EndOfInput);
    }

    let first = input.chars().next().unwrap();
    let qtype = match first {
        '?' => QuantifierType::ZeroOrOne,
        '*' => QuantifierType::ZeroOrMore,
        '+' => QuantifierType::OneOrMore,
        _ => {
            return Err(ParseError::InvalidQuantifier);
        }
    };
    Ok((Quantifier(qtype, QuantifierModifier::Greedy), &input[1..]))
}

#[cfg(test)]
mod test {
    use crate::{parser::*, *};

    #[test]
    fn should_parse_character() {
        let input = "a";
        assert_eq!(parse_expression(input), Ok((character('a'), "")));
    }

    #[test]
    fn should_parse_alternation() {
        let input = "a|b|c";
        assert_eq!(
            parse_expression(input),
            Ok((
                alternation(alternation(character('a'), character('b')), character('c')),
                ""
            ))
        );
    }

    #[test]
    fn should_parse_concatenation() {
        let input = "ab";
        assert_eq!(
            parse_expression(input),
            Ok((concatenation(character('a'), character('b')), ""))
        );
    }

    #[test]
    fn should_parse_repetition() {
        let input = "a*";
        assert_eq!(
            parse_expression(input),
            Ok((
                repetition(
                    character('a'),
                    Quantifier(QuantifierType::ZeroOrMore, QuantifierModifier::Greedy)
                ),
                ""
            ))
        );
    }

    #[test]
    fn should_parse_precedence() {
        let input = "a|(bc)*|d";
        assert_eq!(
            parse_expression(input),
            Ok((
                alternation(
                    alternation(
                        character('a'),
                        repetition(
                            concatenation(character('b'), character('c')),
                            Quantifier(QuantifierType::ZeroOrMore, QuantifierModifier::Greedy)
                        ),
                    ),
                    character('d')
                ),
                ""
            ))
        );
    }

    #[test]
    fn should_reject_invalid() {
        let input = "d++|e";
        assert_eq!(parse_regex(input), None);
    }
}
