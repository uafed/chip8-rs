use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, space0, space1, u8},
    combinator::{all_consuming, map, opt, recognize},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
};

#[derive(Debug)]
pub enum TopLevelStatement {
    FunctionDecl(FunctionDeclaration),
}

#[derive(Debug)]
enum Type {
    Void,
    U8,
    Inferred,
}

#[allow(dead_code, unused)]
#[derive(Debug)]
pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<FunctionParameter>,
    return_type: Type,
    body: Vec<Statement>,
}

#[allow(dead_code, unused)]
#[derive(Debug)]
pub struct FunctionParameter {
    name: String,
    parameter_type: Type,
}

#[allow(dead_code, unused)]
#[derive(Debug)]
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

#[derive(Debug)]
pub enum Statement {
    Return(ExpressionNode),
    If(IfStatement),
    Expression(ExpressionNode),
}

pub fn parse_program(input: &str) -> IResult<&str, Vec<TopLevelStatement>> {
    let (input, (_, top_level_statements, _)) = all_consuming((
        multispace0,
        separated_list1(multispace0, parse_top_level_statement),
        multispace0,
    ))
    .parse(input)?;
    Ok((input, top_level_statements))
}

pub fn parse_top_level_statement(input: &str) -> IResult<&str, TopLevelStatement> {
    map(parse_function_declaration, TopLevelStatement::FunctionDecl).parse(input)
}

pub fn parse_function_declaration(input: &str) -> IResult<&str, FunctionDeclaration> {
    let (input, _) = (tag("fn"), space1).parse(input)?;
    let (input, name) = parse_identifier.parse(input)?;

    let (input, (_, parameters)) = (
        multispace0,
        delimited(tag("("), many0(parse_parameter), tag(")")),
    )
        .parse(input)?;

    let (input, return_type) = opt((
        space0,
        (tag(":"), space0),
        alt((
            map(tag("void"), |_| Type::Void),
            map(tag("u8"), |_| Type::U8),
        )),
    ))
    .parse(input)?;
    let return_type = return_type.map_or(Type::Void, |(_, _, return_type)| return_type);

    let (input, (_, body)) = (
        multispace0,
        delimited(
            (tag("{"), multispace0),
            many0(parse_statement),
            (multispace0, tag("}")),
        ),
    )
        .parse(input)?;

    Ok((
        input,
        FunctionDeclaration {
            name: name.to_string(),
            parameters,
            return_type,
            body,
        },
    ))
}

pub fn parse_parameter(input: &str) -> IResult<&str, FunctionParameter> {
    let (input, (name, parameter_type)) = separated_pair(
        parse_identifier,
        (tag(":"), space0),
        alt((
            map(tag("void"), |_| Type::Void),
            map(tag("u8"), |_| Type::U8),
            // NOTE: not sure if this is how I want this to be handled right now
            map(parse_identifier, |_| Type::Inferred),
        )),
    )
    .parse(input)?;

    Ok((
        input,
        FunctionParameter {
            name: name.to_string(),
            parameter_type,
        },
    ))
}

pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_if_statement,
        map((parse_return_statement, tag(";")), |(statement, _)| {
            statement
        }),
        map((parse_expression, tag(";")), |(expression, _)| {
            Statement::Expression(expression)
        }),
    ))
    .parse(input)
}

pub fn parse_if_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("if").parse(input)?;
    let (input, (_, condition)) =
        (space0, delimited(tag("("), parse_expression, tag(")"))).parse(input)?;

    let (input, (_, body)) = (
        space0,
        delimited(
            (tag("{"), multispace0),
            many0(parse_statement),
            (multispace0, tag("}")),
        ),
    )
        .parse(input)?;

    Ok((input, Statement::If(IfStatement { condition, body })))
}

pub fn parse_return_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("return").parse(input)?;
    let (input, (_, expression)) = (multispace1, parse_expression).parse(input)?;

    Ok((input, Statement::Return(expression)))
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
