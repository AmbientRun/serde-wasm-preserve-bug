mod utils;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TestStruct {
    pub child: TestStructChild,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TestStructChild {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub value: JsValue,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(test: TestStruct) {
    console::log_1(&format!("Hello, {}!", test.child.value.as_f64().unwrap()).into());
    let x = serde_wasm_bindgen::to_value(&test).unwrap();
    let y: TestStruct = serde_wasm_bindgen::from_value(x).unwrap();
    console::log_1(&format!("Hello again, {}!", y.child.value.as_f64().unwrap()).into());
}
