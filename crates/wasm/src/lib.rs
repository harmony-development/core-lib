use js_sys::Map;
use wasm_bindgen::{prelude::*, JsCast};

/*#[wasm_bindgen]
pub fn has_permission(permissions: Map, query: &str) -> Option<bool> {
    let keys = permissions.keys().into_iter();
    let values = permissions.values().into_iter();

    core::permission::has_permission(
        keys.zip(values).flat_map(|(key, value)| {
            let (key, value) = (key.ok()?, value.ok()?);
            Some((key.dyn_ref::<JsString>()?.(), key.as_bool()?))
        }),
        query,
    )
}*/
