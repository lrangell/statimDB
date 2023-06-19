macro_rules! delimited {
    ($start: literal, $content: expr) => {
        format!("{}{}\r\n", $start, $content)
    };
}

trait RespString {
    fn resp(&self) -> String;
}

impl RespString for i32 {
    fn resp(&self) -> String {
        delimited!(":", self)
    }
}
impl RespString for String {
    fn resp(&self) -> String {
        delimited!("+", self)
    }
}
impl<T: RespString> RespString for Vec<T> {
    fn resp(&self) -> String {
        let content = self
            .iter()
            .map(|s| s.resp())
            .collect::<Vec<String>>()
            .join("");
        let size = self.len();
        format!("*{size}{content}\r\n")
    }
}

fn delimited(start: &str, content: impl ToString) -> String {
    format!("{}{}\r\n", start, content.to_string())
}

fn simple_string(s: impl ToString) -> String {
    delimited("+", s)
}
