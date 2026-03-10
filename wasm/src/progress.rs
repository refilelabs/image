use js_sys;
use wasm_bindgen::prelude::*;

/// Calls the progress callback with a `{ progress, message }` JS object.
pub(crate) fn report(cb: &js_sys::Function, progress: f64, message: &str) {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"progress".into(), &JsValue::from_f64(progress)).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &JsValue::from_str(message)).ok();
    let _ = cb.call1(&JsValue::NULL, &obj.into());
}
