#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec};

use harmony_core as hcore;
use js_sys::{Array, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(typescript_custom_section)]
const UINT8: &str = r#"
type uint8 = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 | 176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 | 192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 | 200 | 201 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 | 224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237 | 238 | 239 | 240 | 241 | 242 | 243 | 244 | 245 | 246 | 247 | 248 | 249 | 250 | 251 | 252 | 253 | 254;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<[string, boolean]>")]
    pub type PermissionArray;
    #[wasm_bindgen(typescript_type = "[uint8, uint8, uint8]")]
    pub type RgbColor;
}

#[wasm_bindgen]
pub fn has_permission(permissions: PermissionArray, query: &str) -> Result<Option<bool>, JsValue> {
    let array: Array = permissions
        .dyn_into()
        .map_err(|_| "value not of type 'Array<[string, boolean]>'")?;

    let array_len = array.length() as usize;
    let perms = array.iter().try_fold(
        Vec::with_capacity(array_len),
        |mut tot: Vec<(String, bool)>, value| {
            let tuple: Array = value
                .dyn_into()
                .map_err(|_| "value not of type '[string, boolean]'")?;
            let mut tuple_iter = tuple.iter();

            let key = tuple_iter
                .next()
                .ok_or("expected 'string' type value as first element of array")?;
            let value = tuple_iter
                .next()
                .ok_or("expected 'boolean' type value as second element of array")?;

            let item = (
                key.as_string().ok_or("expected value with type 'string'")?,
                value
                    .as_bool()
                    .ok_or("expected value with type 'boolean'")?,
            );
            tot.push(item);

            Result::<_, JsValue>::Ok(tot)
        },
    )?;

    Ok(hcore::permission::has_permission(perms.into_iter(), query))
}

#[wasm_bindgen]
pub fn encode_rgb(color: RgbColor) -> Result<i32, JsValue> {
    let array: Uint8Array = color
        .dyn_into()
        .map_err(|_| "value not of type '[uint8, uint8, uint8]'")?;
    Ok(hcore::color::encode_rgb([
        array.get_index(0),
        array.get_index(1),
        array.get_index(2),
    ]))
}

#[wasm_bindgen]
pub fn decode_rgb(color: i32) -> RgbColor {
    let decoded = hcore::color::decode_rgb(color);
    Uint8Array::from(decoded.as_ref()).unchecked_into()
}

#[cfg(test)]
pub mod tests {
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_test::*;

    use crate::RgbColor;

    #[wasm_bindgen_test]
    fn decode_rgb() {
        let decoded: Uint8Array = super::decode_rgb(0).unchecked_into();
        let color = [
            decoded.get_index(0),
            decoded.get_index(1),
            decoded.get_index(2),
        ];
        assert_eq!(color, [0; 3])
    }

    #[wasm_bindgen_test]
    fn encode_rgb() {
        let color: RgbColor = Uint8Array::from([0; 3].as_ref()).unchecked_into();
        let encoded = super::encode_rgb(color).unwrap();
        assert_eq!(encoded, 0)
    }

    #[wasm_bindgen_test]
    fn has_permission() {
        let mk_perm_tuple = |m: &str, ok: bool| -> Array {
            [JsValue::from_str(m), JsValue::from_bool(ok)]
                .iter()
                .collect()
        };
        let permissions: Array = [mk_perm_tuple("*", true)].iter().collect();
        let ok = super::has_permission(permissions.unchecked_into(), "messages.send").unwrap();
        assert_eq!(ok, Some(true))
    }
}
