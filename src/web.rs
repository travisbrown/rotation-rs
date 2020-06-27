use image::{guess_format, load_from_memory_with_format, DynamicImage, ImageError};
use js_sys::{Error, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub async fn load_image(url: String) -> Result<DynamicImage, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let global = js_sys::global().unchecked_into::<web_sys::WorkerGlobalScope>();
    let resp_val = JsFuture::from(global.fetch_with_request(&request)).await?;
    let resp: Response = resp_val.dyn_into().unwrap();
    let buf_fut = resp.array_buffer()?;
    let buf = JsFuture::from(buf_fut).await?;

    let uint8_arr = Uint8Array::new(&buf);
    let mut bytes = vec![0; uint8_arr.length() as usize];
    uint8_arr.copy_to(&mut bytes);
    let format = guess_format(&bytes).map_err(err_img_to_js)?;
    load_from_memory_with_format(&bytes, format).map_err(err_img_to_js)
}

fn err_img_to_js(i: ImageError) -> JsValue {
    JsValue::from(Error::new(&format!("Image error: {}", i)))
}
