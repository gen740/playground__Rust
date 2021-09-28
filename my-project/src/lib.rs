use neon::prelude::*;
// use std::vec::*;

// let mut vecoeu = std::collections::VecDeque::new();
// let mut vec = Vec::new();

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn test(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(100))
}

// let fn_arr: [fn; 1] = [tset,];

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("test", test)?;
    Ok(())
}
