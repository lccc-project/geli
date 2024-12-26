pub mod filter;
pub mod lexer;
pub mod parser;

use filter::Filter;
use lexer::Lexer;
use parser::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pipeline {
    pub settings: PipelineSettings,
    pub nodes: Vec<PipelineNode>,
    pub edges: Vec<PipelineEdge>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PipelineSettings {
    pub separate_output_fns: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineNode {
    pub name: String,
    #[serde(flatten)]
    pub body: PipelineNodeBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PipelineNodeBody {
    Input,
    Lexer(Lexer),
    Parser(Parser),
    Filter(Filter),
    Output,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineEdge {
    pub start: String,
    pub end: String,
}
