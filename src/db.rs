use futures::future::{BoxFuture, Future};
use statim_macros::{command, toParams};
use std::borrow::BorrowMut;
use std::pin::{pin, Pin};
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::sync::OnceLock;

use crate::resp;
type Operation = fn(Vec<resp::Expr>) -> dyn Future<Output = String>;

lazy_static! {
    static ref HMAP: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}
type Op = fn(&[resp::Expr]) -> BoxFuture<Result<String>>;

static TABLE: OnceLock<Mutex<HashMap<&'static str, Op>>> = OnceLock::new();

#[statim_macros::command]
async fn set(key: String, value: String) -> Result<String> {
    HMAP.lock().await.insert(key, value);
    todo!()
}
#[statim_macros::command]
async fn get(key: String) -> Result<String> {
    let r = HMAP.lock().await.get(&key);
    todo!()
}

statim_macros::build_dispatch_table!();
