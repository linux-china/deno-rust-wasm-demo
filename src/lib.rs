use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use boa::{Context, JsValue as BoaJsValue};
use once_cell::sync::OnceCell;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut CONTEXT: OnceCell<Mutex<Context>> = OnceCell::new();
static mut JS_HELLO_FN: Option<BoaJsValue> = None;

#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    unsafe {
        let mut context = Context::new();
        let js_code = "function hello() {return 'hello' ;} hello;";
        JS_HELLO_FN = context.eval(js_code).ok();
        CONTEXT.set(Mutex::new(context)).unwrap();
    }
}

#[wasm_bindgen]
pub fn hello_boa(name: String) -> String {
    unsafe {
        let mut ctx = CONTEXT.get().unwrap().lock().unwrap();
        let handler = JS_HELLO_FN.as_ref().unwrap();
        let result = handler.as_object().unwrap().call(handler, &vec![], &mut ctx).unwrap();
        format!("{} {} from Boa", result.as_string().unwrap().to_string(), name)
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn hello(name: String) -> String {
    return format!("Hello {}", name);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ip {
    pub origin: String,
}

#[wasm_bindgen]
pub async fn fetch_ip() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = format!("https://httpbin.org/ip");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    // Use serde to parse the JSON into a struct.
    let ip_info: Ip = json.into_serde().unwrap();
    // Send the `IP` struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&ip_info).unwrap())
}


#[test]
fn test_add() {
    println!("{}", add(1_i32, 2_i32));
}

#[test]
fn test_hello() {
    println!("{}", hello("Jackie".into()));
}
