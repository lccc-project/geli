use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Lexer {
    pub rules: Vec<LexerRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LexerRule {
    pub name: String,
    #[serde(flatten)]
    pub body: LexerRuleBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LexerRuleBody {
    Literal {
        value: String,
    },
    Regex {
        value: String,
    },
    Group {
        start: String,
        end: String,
    },
    String {
        start: String,
        end: String,
        escaped: Vec<char>,
    },
}
