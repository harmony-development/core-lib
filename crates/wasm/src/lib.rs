use js_sys::Map;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn has_permission(permissions: Map, query: &str) -> Option<bool> {
    let keys = permissions.keys().into_iter();
    let values = permissions.values().into_iter();

    let perms = keys
        .zip(values)
        .flat_map(|(key, value)| {
            let (key, value) = (key.ok()?, value.ok()?);
            Some((key.as_string()?, value.as_bool()?))
        })
        .collect::<Vec<_>>();

    core::permission::has_permission(perms.iter().map(|(m, o)| (m.as_str(), *o)), query)
}

#[wasm_bindgen]
pub fn encode_rgb(color: &[u8]) -> i32 {
    core::color::encode_rgb([
        color.get(0).copied().unwrap_or(0),
        color.get(1).copied().unwrap_or(0),
        color.get(2).copied().unwrap_or(0),
    ])
}

#[wasm_bindgen]
pub fn decode_rgb(color: i32) -> Box<[u8]> {
    Box::new(core::color::decode_rgb(color))
}
