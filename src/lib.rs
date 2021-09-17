mod buffer;
mod image_gen;

use lazy_static::lazy_static;
use neon::prelude::*;

lazy_static! {
    static ref THREAD_POOL: rayon::ThreadPool = init_pool();
}

const RAYON_THREAD_NUM: usize = 8;
fn init_pool() -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(RAYON_THREAD_NUM)
        .build()
        .unwrap()
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("jsBufferTo", buffer::jsbuffer_to)?;
    cx.export_function("bufferFromRs", buffer::buffer_from_rs)?;
    cx.export_function("imageGen", image_gen::rayon_image_gen)?;
    Ok(())
}
