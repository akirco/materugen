#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use crate::MaterialYou;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn generate_from_hex(hex_color: &str, theme_variant: &str) -> Result<String, JsError> {
    let material = MaterialYou::from_hex(hex_color).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(material.generate_theme_json(theme_variant))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn generate_random(theme_variant: &str) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let hex_color = format!(
        "#{:02X}{:02X}{:02X}",
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255)
    );
    let material = MaterialYou::from_hex(&hex_color).unwrap();
    material.generate_theme_json(theme_variant)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn generate_from_image_bytes(
    image_data: &[u8],
    theme_variant: &str
) -> Result<String, JsError> {
    use image::load_from_memory;
    let img = load_from_memory(image_data).map_err(|e| JsError::new(&e.to_string()))?;
    let material = MaterialYou::from_image(img).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(material.generate_theme_json(theme_variant))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_build_info() -> String {
    format!(
        "Version: {}\nTarget: wasm32-unknown-unknown\nProfile: {}\nFeatures: wasm",
        env!("CARGO_PKG_VERSION"),
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    )
}

#[cfg(all(test, feature = "wasm"))]
mod tests {
    use super::*;

    #[test]
    fn test_generate_from_hex() {
        let result = generate_from_hex("#FF0000", "light");
        assert!(result.is_ok());
        let theme_json = result.unwrap();
        assert!(theme_json.contains("\"type\": \"light\""));
    }

    #[test]
    fn test_generate_random() {
        let theme_json = generate_random("dark");
        assert!(theme_json.contains("\"type\": \"dark\""));
    }
}
