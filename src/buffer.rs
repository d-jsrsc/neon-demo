use neon::prelude::*;

pub fn jsbuffer_to(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut b: Handle<JsBuffer> = cx.argument(0)?;
    // 1
    // let (x, y) = cx.borrow(&b, |data| {
    //     let slice = data.as_slice::<u8>();
    //     println!("{:?}", slice);
    //     (slice[0], slice[1])
    // });
    // println!("({}, {})", x, y);
    // 2
    // cx.borrow_mut(&mut b, |data| {
    //     let slice = data.as_mut_slice::<u8>();
    //     slice[0] /= 2;
    //     slice[1] *= 2;
    // });
    // 3
    let buf = cx.borrow_mut(&mut b, |data| data.as_mut_slice::<u8>());
    buf[0] /= 2;
    buf[1] *= 2;

    Ok(cx.undefined())
}

pub fn buffer_from_rs(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    // 1 返回 buffer
    // JsBuffer::new(&mut cx, 8)
    // 2 返回 u8 Vec
    let buf: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff];
    let buf = JsBuffer::external(&mut cx, buf);
    Ok(buf)
}
