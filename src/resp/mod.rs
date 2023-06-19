pub mod parser;
pub mod serializer;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    String(String),
    Integer(i64),
    Array(Vec<Expr>),
    Null,
    Error(String),
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::String(s) => s.to_string(),
            Expr::Integer(_) => todo!(),
            Expr::Array(_) => todo!(),
            Expr::Null => todo!(),
            Expr::Error(_) => todo!(),
        }
    }
}

unsafe impl Send for Expr {}

unsafe impl Sync for Expr {}

impl From<Expr> for String {
    fn from(value: Expr) -> Self {
        match value {
            Expr::String(s) => s,
            Expr::Integer(i) => i.to_string(),
            Expr::Array(_) => todo!(),
            Expr::Null => todo!(),
            Expr::Error(_) => todo!(),
        }
    }
}

trait Serialize {
    fn resp_string(self) -> String;
}

// impl Serialize for  {}
