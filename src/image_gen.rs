// use image::EncodableLayout;
use crate::THREAD_POOL;
use image::DynamicImage;
use neon::prelude::*;
use std::time::Instant;

pub fn rayon_image_gen(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_object = cx.argument::<JsObject>(0)?;
    let imgx = js_object
        .get(&mut cx, "imgx")?
        .downcast::<JsNumber, _>(&mut cx)
        .unwrap_or_else(|_| JsNumber::new(&mut cx, 0))
        .value(&mut cx);
    let imgy = js_object
        .get(&mut cx, "imgy")?
        .downcast::<JsNumber, _>(&mut cx)
        .unwrap_or_else(|_| JsNumber::new(&mut cx, 0))
        .value(&mut cx);

    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let queue = cx.channel();

    let num = THREAD_POOL.current_num_threads();
    println!("num: {:?}", num);
    let start = Instant::now();

    // std::thread::spawn(move || { 200+
    // rayon::spawn(move || { 100+
    // THREAD_POOL.install(move || {
    // √
    THREAD_POOL.spawn(move || {
        println!("++ 0 is: {:?}", start.elapsed());
        let start = Instant::now();
        let image_buf = image_buf(imgx as _, imgy as _);
        println!("++ 1 is: {:?}", start.elapsed());
        let start = Instant::now();
        queue.send(move |mut cx| {
            println!("++ 2 is: {:?}", start.elapsed());
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let buf = JsBuffer::external(&mut cx, image_buf);
            let args = vec![
                // buf.upcast::<JsValue>(),
                // JsNumber::new(&mut cx, 1).upcast::<JsValue>(),
                // cx.number(result as f64).upcast::<JsValue>(),
                cx.null().upcast::<JsValue>(),
                buf.upcast::<JsValue>(),
            ];
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
}

fn image_buf(imgx: u32, imgy: u32) -> Vec<u8> {
    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    let dm = DynamicImage::ImageRgb8(imgbuf);
    let mut bytes: Vec<u8> = Vec::new();
    dm.write_to(&mut bytes, image::ImageOutputFormat::Jpeg(75))
        .unwrap();
    bytes
}
