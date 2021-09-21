use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<[string, bool]>")]
    pub type PermissionArray;
}

#[wasm_bindgen]
pub fn has_permission(permissions: PermissionArray, query: &str) -> Result<Option<bool>, JsValue> {
    let array = permissions
        .dyn_into::<Array>()
        .map_err(|_| "value not of type 'Array<[string, bool]>'")?;

    let array_len = array.length() as usize;
    let perms = array.iter().try_fold(
        Vec::with_capacity(array_len),
        |mut tot: Vec<(String, bool)>, value| {
            let tuple: Array = value
                .dyn_into()
                .map_err(|_| "value not of type '[string, bool]'")?;
            let mut tuple_iter = tuple.iter();

            let key = tuple_iter
                .next()
                .ok_or("expected 'string' type value as first element of array")?;
            let value = tuple_iter
                .next()
                .ok_or("expected 'bool' type value as second element of array")?;

            let item = (
                key.as_string().ok_or("expected value with type 'string'")?,
                value.as_bool().ok_or("expected value with type 'bool'")?,
            );
            tot.push(item);

            Result::<_, JsValue>::Ok(tot)
        },
    )?;

    Ok(harmony_core::permission::has_permission(
        perms.iter().map(|(m, o)| (m.as_str(), *o)),
        query,
    ))
}

#[wasm_bindgen]
pub fn encode_rgb(color: &[u8]) -> i32 {
    harmony_core::color::encode_rgb([
        color.get(0).copied().unwrap_or(0),
        color.get(1).copied().unwrap_or(0),
        color.get(2).copied().unwrap_or(0),
    ])
}

#[wasm_bindgen]
pub fn decode_rgb(color: i32) -> Box<[u8]> {
    Box::new(harmony_core::color::decode_rgb(color))
}
