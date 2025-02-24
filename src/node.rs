use std::{fmt, ops::Range, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Pos,
    Neg,
    Abs,
    Floor,
    Ceil,
    Round,
    Sqrt,
    Cbrt,
    Fort,
    Degree,
    Fact,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "×"),
            Self::Div => write!(f, "÷"),
            Self::Mod => write!(f, "mod"),
            Self::Pow => write!(f, "^"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Number(Rc<str>),
    Identifier(Rc<str>),
    Assignment(Rc<str>, Box<Node>),
    Unary(UnaryOp, Box<Node>),
    Binary(Box<Node>, BinaryOp, Box<Node>),
    FnDef(Rc<str>, Vec<Rc<str>>, Box<Node>),
    Call(Rc<str>, Vec<Node>),
    Statements(Vec<Node>),
    EOF,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::Identifier(name) => write!(f, "{}", name),
            Self::Assignment(name, node) => write!(f, "({} = {})", name, node),
            Self::Unary(op, node) => match op {
                UnaryOp::Pos => write!(f, "(+{})", node),
                UnaryOp::Neg => write!(f, "(-{})", node),
                UnaryOp::Abs => write!(f, "|{}|", node),
                UnaryOp::Floor => write!(f, "⌊{}⌋", node),
                UnaryOp::Ceil => write!(f, "⌈{}⌉", node),
                UnaryOp::Round => write!(f, "⌊{}⌉", node),
                UnaryOp::Sqrt => write!(f, "(√{})", node),
                UnaryOp::Cbrt => write!(f, "(∛{})", node),
                UnaryOp::Fort => write!(f, "(∜{})", node),
                UnaryOp::Degree => write!(f, "({}°)", node),
                UnaryOp::Fact => write!(f, "({}!)", node),
            },
            Self::Binary(left, op, right) => write!(f, "({} {} {})", left, op, right),
            Self::FnDef(name, args, body) => write!(
                f,
                "fn {}({}) {{\n  {}\n}}",
                name,
                args.iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", "),
                body
            ),
            Self::Call(name, args) => write!(
                f,
                "{}({})",
                name,
                args.iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Statements(nodes) => write!(
                f,
                "{{\n  {}\n}}",
                nodes
                    .iter()
                    .map(|node| format!("{}", node))
                    .collect::<Vec<String>>()
                    .join("\n  ")
            ),
            Self::EOF => write!(f, "<eof>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub ty: NodeType,
    pub range: Range<usize>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ty)
    }
}
