#![allow(unused)]
pub mod db;
pub mod resp;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::init_table().await;
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                let s = String::from_utf8(buf[0..n].to_vec()).unwrap();
                println!("{}", s);

                if let resp::Expr::Array(request) = resp::parser::parse(&s).unwrap() {
                    let (cmd, args) = request.split_first().unwrap();
                    let db_table = db::TABLE.get().unwrap().lock().await;
                    println!("{:?}", db_table);

                    let cmd_fn = db_table.get(dbg!(cmd.to_string().as_str())).unwrap();
                    let res = dbg!(cmd_fn(args).await);
                    socket.write_all(&res.into_bytes()).await;
                }
            }
        });
    }
}
