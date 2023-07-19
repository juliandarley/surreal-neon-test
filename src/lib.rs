use std::ops::Deref;

use neon::prelude::*;
use neon::{
    result::{JsResult, NeonResult},
    types::JsPromise,
};
use once_cell::sync::OnceCell;
use surrealdb::sql::Value;
use surrealdb::Connection;
use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};
use tokio::runtime::Runtime;

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

fn new_db(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let address = cx.argument::<JsString>(0)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let resp = Surreal::new::<File>(address.as_str()).await;

        deferred.settle_with(&channel, move |mut cx| {
            let db: Surreal<Db> = resp.or_else(|err| cx.throw_error(err.to_string()))?;

            return Ok(cx.boxed(DBWrapper(db)));
        })
    });

    Ok(promise)
}

fn query(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let db = cx.argument::<JsBox<DBWrapper<Db>>>(0)?.deref().0.clone();
    let query = cx.argument::<JsString>(1)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let resp = db.query(query).await;

        deferred.settle_with(&channel, move |mut cx| {
            let mut response = resp.or_else(|err| cx.throw_error(err.to_string()))?;

            let num_statements = response.num_statements();

            let value: Value = if num_statements > 1 {
                let mut output = Vec::<Value>::with_capacity(num_statements);
                for index in 0..num_statements {
                    output.push(match response.take(index) {
                        Ok(v) => v,
                        Err(e) => e.to_string().into(),
                    });
                }
                Value::from(output)
            } else {
                response
                    .take(0)
                    .or_else(|err| cx.throw_error(err.to_string()))?
            };

            let json = serde_json::to_string(&value.into_json())
                .or_else(|err| cx.throw_error(err.to_string()))?;

            return Ok(cx.string(&json));
        })
    });

    Ok(promise)
}

fn use_ns(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let db = cx.argument::<JsBox<DBWrapper<Db>>>(0)?.deref().0.clone();
    let ns = cx.argument::<JsString>(1)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let resp = db.use_ns(ns).await;

        deferred.settle_with(&channel, move |mut cx| {
            resp.or_else(|err| cx.throw_error(err.to_string()))?;

            return Ok(cx.undefined());
        })
    });

    Ok(promise)
}

fn use_db(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let db = cx.argument::<JsBox<DBWrapper<Db>>>(0)?.deref().0.clone();
    let db_name = cx.argument::<JsString>(1)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let resp = db.use_db(db_name).await;

        deferred.settle_with(&channel, move |mut cx| {
            resp.or_else(|err| cx.throw_error(err.to_string()))?;

            return Ok(cx.undefined());
        })
    });

    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("new_db", new_db)?;
    cx.export_function("query", query)?;
    cx.export_function("use_ns", use_ns)?;
    cx.export_function("use_db", use_db)?;
    Ok(())
}

struct DBWrapper<C: Connection>(Surreal<C>);

impl Finalize for DBWrapper<Db> {}
