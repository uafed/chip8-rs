use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space0, space1, u8},
    combinator::{map, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, pair},
};

pub enum TopLevelStatement {
    FunctionDecl,
}

#[derive(Debug)]
enum Type {
    Void,
    U8,
    Inferred,
}

#[allow(dead_code, unused)]
pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<FunctionParameter>,
    return_type: Type,
    body: Vec<Statement>,
}

#[allow(dead_code, unused)]
pub struct FunctionParameter {
    name: String,
    parameter_type: Type,
}

#[allow(dead_code, unused)]
pub struct IfStatement {
    condition: ExpressionNode,
    body: Vec<Statement>,
}

#[allow(dead_code, unused)]
#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<ExpressionNode>,
}

#[allow(dead_code, unused)]
#[derive(Debug)]
pub struct Literal {
    // NOTE: Not sure of this is the best way to do this yet.
    text: String,
    literal_type: Type,
}

#[derive(Debug)]
pub enum BinaryOperator {
    LessThan,
    Minus,
    Plus,
}

#[allow(dead_code, unused)]
#[derive(Debug)]
pub struct BinaryExpressionNode {
    left: Box<ExpressionNode>,
    right: Box<ExpressionNode>,
    operator: BinaryOperator,
}

#[derive(Debug)]
pub enum ExpressionNode {
    FunctionCall(FunctionCall),
    Identifier(String),
    Literal(Literal),
    Binary(BinaryExpressionNode),
}

pub enum Statement {
    ReturnStatement(ExpressionNode),
    IfStatement(IfStatement),
    Expression(ExpressionNode),
}

pub fn parse_program(input: &str) -> IResult<&str, ExpressionNode> {
    //TODO: remove
    let (input, expr) = parse_expression(input)?;

    Ok((input, expr))
}

pub fn parse_top_level_statement() {
    todo!()
}

pub fn parse_function_declaration() {
    // "fn", <identifier>, args list, braces, body
    todo!()
}

pub fn parse_parameter() {
    // Identifier
    // Type
    todo!()
}

pub fn parse_statement() {
    // Statements:
    // If statement
    // variable declaration
    // Return statement
    todo!()
}

pub fn parse_if_statement(_input: &str) -> IResult<&str, Statement> {
    todo!()
}

pub fn parse_expression(input: &str) -> IResult<&str, ExpressionNode> {
    alt((
        map(parse_binary_expression, ExpressionNode::Binary),
        map(parse_function_call, ExpressionNode::FunctionCall),
        map(parse_identifier, |expr| {
            ExpressionNode::Identifier(expr.to_string())
        }),
        map(parse_literal_node, ExpressionNode::Literal),
    ))
    .parse(input)
}

pub fn parse_binary_expression(input: &str) -> IResult<&str, BinaryExpressionNode> {
    // NOTE: need to account for left recursion, this is probably the best way right now
    let (input, left) = alt((
        map(parse_function_call, ExpressionNode::FunctionCall),
        map(parse_identifier, |expr| {
            ExpressionNode::Identifier(expr.to_string())
        }),
        map(parse_literal_node, ExpressionNode::Literal),
    ))
    .parse(input)?;

    let (input, _) = space0.parse(input)?;

    let (input, operator) = alt((
        map(tag("<"), |_| BinaryOperator::LessThan),
        map(tag("-"), |_| BinaryOperator::Minus),
        map(tag("+"), |_| BinaryOperator::Plus),
    ))
    .parse(input)?;

    let (input, _) = space0.parse(input)?;

    let (input, right) = parse_expression(input)?;

    Ok((
        input,
        BinaryExpressionNode {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        },
    ))
}

pub fn parse_function_call(input: &str) -> IResult<&str, FunctionCall> {
    let (input, name) = parse_identifier(input)?;
    let (input, arguments) = delimited(
        tag("("),
        separated_list0(tag(","), parse_expression),
        tag(")"),
    )
    .parse(input)?;

    Ok((
        input,
        FunctionCall {
            name: name.to_string(),
            arguments,
        },
    ))
}

pub fn parse_literal_node(input: &str) -> IResult<&str, Literal> {
    // TODO: update handling later on
    alt((map(u8, |text| Literal {
        text: text.to_string(),
        literal_type: Type::U8,
    }),))
    .parse(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    // must start with letter, any case
    // cannot start with number
    // can have underscores
    recognize(pair(alpha1, many0(alt((tag("_"), alpha1))))).parse(input)
}
