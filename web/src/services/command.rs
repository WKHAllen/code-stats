use js_sys::{Function, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub async fn tauri_command<T>(command: &str, args: &T) -> Result<JsValue, JsValue>
where
    T: serde::ser::Serialize + ?Sized,
{
    let tauri = web_sys::window().unwrap().get("__TAURI__").unwrap();
    let invoke = Reflect::get(&tauri.into(), &"invoke".into()).unwrap();
    let invoke_function = invoke.dyn_ref::<Function>().unwrap();

    let js_args = JsValue::from_serde(&args).unwrap();

    let response = invoke_function
        .call2(&invoke_function, &command.into(), &js_args)
        .unwrap();
    let response_promise = response.dyn_into::<Promise>().unwrap();
    let response_future = JsFuture::from(response_promise);
    response_future.await
}
