#[cfg(feature = "wasm")]
#[derive(tsify::Tsify, serde::Deserialize, serde::Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Settings {
    #[serde(rename = "svg")]
    Svg(SvgSettings),
}

#[cfg(not(feature = "wasm"))]
#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Settings {
    Svg(SvgSettings),
}

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct SvgSettings {
    pub width: u32,
    pub height: u32,
}

impl Default for SvgSettings {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
        }
    }
}
