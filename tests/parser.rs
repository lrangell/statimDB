use statim_db::resp::*;

#[test]
fn string_test() {
    let (_, exp) = string("$5\r\nhello\r\n").unwrap();
    assert_eq!(exp, Expr::String("hello".into()));
}
#[test]
fn empty_string() {
    let (_, exp) = string("$0\r\n\r\n").unwrap();
    assert_eq!(exp, Expr::String("".into()));
}

#[test]
fn null_string() {
    let (_, exp) = null("$-1\r\n").unwrap();
    assert_eq!(exp, Expr::Null);
}
#[test]
fn simple_string_test() {
    let (_, exp) = simple_string("+OK\r\n").unwrap();
    assert_eq!(exp, Expr::String("OK".into()));
}

#[test]
fn integer_test() {
    let (_, exp) = integer(":1000\r\n").unwrap();
    assert_eq!(exp, Expr::Integer(1000));
}
#[test]
fn array_with_null() {
    let (_, exp) = array("*3\r\n$5\r\nhello\r\n$-1\r\n$5\r\nworld\r\n").unwrap();
    assert_eq!(
        exp,
        Expr::Array(vec![
            Expr::String("hello".into()),
            Expr::Null,
            Expr::String("world".into())
        ])
    );
}
#[test]
fn empty_array() {
    let (_, exp) = array("*0\r\n").unwrap();
    assert_eq!(exp, Expr::Array(vec![]));
}
#[test]
fn integer_array() {
    let (_, exp) = array("*2\r\n:1\r\n:2\r\n").unwrap();
    assert_eq!(exp, Expr::Array(vec![Expr::Integer(1), Expr::Integer(2)]));
}
#[test]
fn string_array() {
    let (_, exp) = array("*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n").unwrap();
    assert_eq!(
        exp,
        Expr::Array(vec![
            Expr::String("hello".into()),
            Expr::String("world".into())
        ])
    );
}

#[test]
fn error_test() {
    let (_, exp) = array("*2\r\n:1\r\n:2\r\n").unwrap();
    assert_eq!(exp, Expr::Array(vec![Expr::Integer(1), Expr::Integer(2)]));
}
