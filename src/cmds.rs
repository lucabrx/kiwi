use std::sync::Arc;

use crate::{db::Db, error::DBError, res::Val};

pub async fn get(args: Vec<String>, db: &Arc<Db>) -> Result<Val, DBError> {
    if let Some(arg) = args.first() {
        match db.get(arg.as_str()).await {
            Ok(Some(res)) => Ok(Val::BulkString(format!("\"{}\"", res.value))),
            Ok(None) => Ok(Val::NullString()),
            Err(e) => Err(e),
        }
    } else {
        Err(DBError::InvalidArg)
    }
}

pub async fn set(mut args: Vec<String>, db: &Arc<Db>) -> Result<Val, DBError> {
    if args.len() < 2 {
        return Err(DBError::InvalidArg);
    }

    let key = args.remove(0);
    let value = args.remove(0);
    let mut ttl: u64 = 0;

    if !args.is_empty() {
        // Quick implementation of TTL handling; adapt as needed for EX, PX, etc.
        if args[0].eq_ignore_ascii_case("EX") {
            ttl = args.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
        }
    }

    db.set(&key, value, ttl).await?;
    Ok(Val::SimpleString("OK".to_string()))
}

pub async fn del(args: Vec<String>, db: &Arc<Db>) -> Result<Val, DBError> {
    if args.is_empty() {
        return Err(DBError::InvalidArg);
    }

    let mut count = 0;

    for key in args.iter() {
        if db.del(key).await.is_ok() {
            count += 1;
        }
    }

    Ok(Val::Int(count))
}

pub fn quit() -> Val {
    Val::SimpleString("OK".to_string())
}

pub fn ping() -> Val {
    Val::SimpleString("PONG".to_string())
}

pub fn echo(args: Vec<String>) -> Val {
    if let Some(arg) = args.first() {
        Val::BulkString(arg.clone())
    } else {
        Val::NullString()
    }
}
