mod parser;

enum Regex {
    Character(char),
    Concatenation(Box<Regex>, Box<Regex>),
    Alternation(Box<Regex>, Box<Regex>),
    Repetition(Box<Regex>, Quantifier)
}

struct Quantifier(QuantifierType, QuantifierModifier);

enum QuantifierModifier {
    Greedy,
    Nongreedy
}

enum QuantifierType {
    ZeroOrOne,
    OneOrMore,
    ZeroOrMore
}
