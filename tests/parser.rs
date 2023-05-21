use statim_db::resp::*;

#[test]
fn string_test() {
    let exp = parse("$5\r\nhello\r\n").unwrap();
    assert_eq!(exp, Expr::String("hello".into()));
}
#[test]
fn empty_string() {
    let exp = parse("$0\r\n\r\n").unwrap();
    assert_eq!(exp, Expr::String("".into()));
}

#[test]
fn null_string() {
    let exp = parse("$-1\r\n").unwrap();
    assert_eq!(exp, Expr::Null);
}
#[test]
fn simple_string_test() {
    let exp = parse("+OK\r\n").unwrap();
    assert_eq!(exp, Expr::String("OK".into()));
}

#[test]
fn integer_test() {
    let exp = parse(":1000\r\n").unwrap();
    assert_eq!(exp, Expr::Integer(1000));
}
#[test]
fn array_with_null() {
    let exp = parse("*3\r\n$5\r\nhello\r\n$-1\r\n$5\r\nworld\r\n").unwrap();
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
    let exp = parse("*0\r\n").unwrap();
    assert_eq!(exp, Expr::Array(vec![]));
}
#[test]
fn integer_array() {
    let exp = parse("*2\r\n:1\r\n:2\r\n").unwrap();
    assert_eq!(exp, Expr::Array(vec![Expr::Integer(1), Expr::Integer(2)]));
}
#[test]
fn string_array() {
    let exp = parse("*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n").unwrap();
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
    let exp = parse("*2\r\n:1\r\n:2\r\n").unwrap();
    assert_eq!(exp, Expr::Array(vec![Expr::Integer(1), Expr::Integer(2)]));
}
