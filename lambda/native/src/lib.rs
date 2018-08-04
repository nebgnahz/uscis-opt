#[macro_use]
extern crate neon;
extern crate uscis;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    // Ok(cx.string("hello node"))
    let string = uscis::crawl_one(1890230606).unwrap();
    Ok(cx.string(&string))
    // Ok(cx.string("hello rust"))
}

register_module!(mut cx, { cx.export_function("hello", hello) });
