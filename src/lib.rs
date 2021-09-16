mod buffer;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("jsBufferTo", buffer::jsbuffer_to)?;
    cx.export_function("bufferFromRs", buffer::buffer_from_rs)?;
    Ok(())
}
