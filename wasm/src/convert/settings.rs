#[derive(tsify::Tsify, serde::Deserialize, serde::Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Settings {
    #[serde(rename = "svg")]
    Svg(SvgSettings),
}

#[derive(tsify::Tsify, serde::Deserialize, serde::Serialize, Copy, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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
