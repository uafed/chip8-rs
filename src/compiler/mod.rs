use nom::IResult;

pub enum TopLevelStatement {
    FunctionDecl,
}

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
pub struct FunctionCall {
    name: String,
    arguments: Vec<ExpressionNode>,
}

#[allow(dead_code, unused)]
pub struct Literal {
    // NOTE: Not sure of this is the best way to do this yet.
    text: String,
    literal_type: Type,
}

pub enum BinaryOperator {
    LessThan,
    Minus,
    Plus,
}

#[allow(dead_code, unused)]
pub struct BinaryExpression {
    left: Box<ExpressionNode>,
    right: Box<ExpressionNode>,
    operator: BinaryOperator,
}

pub enum ExpressionNode {
    FunctionCallNode(FunctionCall),
    IdentifierNode(String),
    LiteralNode(Literal),
    BinaryNode(BinaryExpression),
}

pub enum Statement {
    ReturnStatement(ExpressionNode),
    IfStatement(IfStatement),
    Expression(ExpressionNode),
}

pub fn parse_program() {
    // NOTE: Just trying to see for now if I have enough info to "represent" a
    // very simple program
    FunctionDeclaration {
        name: "main".to_string(),
        return_type: Type::Void,
        parameters: vec![],
        body: vec![Statement::Expression(ExpressionNode::FunctionCallNode(
            FunctionCall {
                name: "fibonacci".to_string(),
                arguments: vec![ExpressionNode::LiteralNode(Literal {
                    text: "10".to_string(),
                    literal_type: Type::Inferred,
                })],
            },
        ))],
    };
    FunctionDeclaration {
        name: "fibonacci".to_string(),
        return_type: Type::U8,
        parameters: vec![FunctionParameter {
            name: "value".to_string(),
            parameter_type: Type::U8,
        }],
        body: vec![
            Statement::IfStatement(IfStatement {
                condition: ExpressionNode::BinaryNode(BinaryExpression {
                    left: Box::new(ExpressionNode::IdentifierNode("value".to_string())),
                    right: Box::new(ExpressionNode::LiteralNode(Literal {
                        text: "2".to_string(),
                        literal_type: Type::Inferred,
                    })),
                    operator: BinaryOperator::LessThan,
                }),
                body: vec![Statement::ReturnStatement(ExpressionNode::IdentifierNode(
                    "value".to_string(),
                ))],
            }),
            Statement::ReturnStatement(ExpressionNode::BinaryNode(BinaryExpression {
                left: Box::new(ExpressionNode::FunctionCallNode(FunctionCall {
                    name: "fibonacci".to_string(),
                    arguments: vec![ExpressionNode::BinaryNode(BinaryExpression {
                        left: Box::new(ExpressionNode::IdentifierNode("value".to_string())),
                        right: Box::new(ExpressionNode::LiteralNode(Literal {
                            text: "1".to_string(),
                            literal_type: Type::Inferred,
                        })),
                        operator: BinaryOperator::Minus,
                    })],
                })),
                right: Box::new(ExpressionNode::FunctionCallNode(FunctionCall {
                    name: "fibonacci".to_string(),
                    arguments: vec![ExpressionNode::BinaryNode(BinaryExpression {
                        left: Box::new(ExpressionNode::IdentifierNode("value".to_string())),
                        right: Box::new(ExpressionNode::LiteralNode(Literal {
                            text: "2".to_string(),
                            literal_type: Type::Inferred,
                        })),
                        operator: BinaryOperator::Minus,
                    })],
                })),
                operator: BinaryOperator::Plus,
            })),
        ],
    };
    todo!()
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
