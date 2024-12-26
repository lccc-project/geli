use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser {
    pub rules: Vec<ParserRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParserRule {
    pub name: String,
    #[serde(flatten)]
    pub body: ParserRuleBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParserRuleBody {
    // SuperType: TypeA | TypeB | TypeC
    SuperType {
        options: Vec<String>, // -> ParserRule
    },
    // Structural: RuleA* 'text' RuleB
    Structural {
        fragments: Vec<ParserRuleFragment>,
    },
    PassThrough,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParserRuleFragment {
    pub name: Option<String>,
    #[serde(flatten)]
    pub body: ParserRuleFragmentBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParserRuleFragmentBody {
    // ',', 'pub', 'fn'
    Literal {
        value: String,
    },
    // Comma, FatArrow, Number (in general, refers to element in preceding node)
    Token {
        rule: String, // -> LexerRule
    },
    // Function, ParamList, CodeBlock (in general, refers to element in this parser)
    Parser {
        rule: String, // -> ParserRule
    },
    // Parens(Expr), etc
    Group {
        rule: String,
        body: Vec<ParserRuleFragment>, // *NOT Self* (as the individual parts are nameable)
    },
    Optional {
        inner: Box<Self>,
    },
    Repeat {
        one_or_more: bool,
        body: Vec<Self>,
    },
    Precedence {
        atom: String, // -> ParserRule
        rules: Vec<PrecedenceRule>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecedenceRule {
    pub rule: String, // -> ParserRule
    pub precedence: i32,
    pub associativity: Associativity,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Associativity {
    Left,
    Right,
    None,
}
