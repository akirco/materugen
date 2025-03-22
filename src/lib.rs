use color_thief::{ get_palette, ColorFormat };
use image::DynamicImage;
use material_you_palette::utils::{ string::argb_from_hex, string::hex_from_argb, theme::Theme };

pub mod error;
pub mod wasm;

pub struct MaterialYou {
    theme: Theme,
}

impl MaterialYou {
    pub fn from_image(image: DynamicImage) -> Result<Self, error::Error> {
        let (buffer, colorformat) = get_image_buffer(image)?;
        let color_palette = get_palette(&buffer, colorformat, 8, 2)?;
        let primary_color = color_palette
            .first()
            .ok_or_else(|| error::Error::Custom("Failed to extract primary color".to_string()))?;

        let theme = Theme::from_source_color([
            255,
            primary_color.r,
            primary_color.g,
            primary_color.b,
        ]);

        Ok(Self { theme })
    }

    pub fn from_hex(hex_color: &str) -> Result<Self, error::Error> {
        let argb = parse_hex_color(hex_color)?;
        let theme = Theme::from_source_color(argb);
        Ok(Self { theme })
    }

    pub fn generate_theme_json(&self, theme_variant: &str) -> String {
        let scheme = if theme_variant == "dark" {
            &self.theme.schemes.dark
        } else {
            &self.theme.schemes.light
        };

        generate_theme_json(theme_variant, &scheme)
    }
}

fn parse_hex_color(hex: &str) -> Result<[u8; 4], error::Error> {
    std::panic
        ::catch_unwind(std::panic::AssertUnwindSafe(|| { argb_from_hex(hex.to_string()) }))
        .map_err(|_| error::Error::InvalidHexColor)
}

fn get_image_buffer(image: DynamicImage) -> Result<(Vec<u8>, ColorFormat), error::Error> {
    match image {
        DynamicImage::ImageRgb8(buf) => Ok((buf.to_vec(), ColorFormat::Rgb)),
        DynamicImage::ImageRgba8(buf) => Ok((buf.to_vec(), ColorFormat::Argb)),
        _ => Err(error::Error::Custom("Unsupported image format".to_string())),
    }
}

fn generate_theme_json(
    theme_variant: &str,
    scheme: &material_you_palette::scheme::Scheme
) -> String {
    format!(
        r#"{{
    "type": "{theme_variant}",
    "primary": "{}",
    "onPrimary": "{}",
    "primaryContainer": "{}",
    "onPrimaryContainer": "{}",
    "secondary": "{}",
    "onSecondary": "{}",
    "secondaryContainer": "{}",
    "onSecondaryContainer": "{}",
    "tertiary": "{}",
    "onTertiary": "{}",
    "tertiaryContainer": "{}",
    "onTertiaryContainer": "{}",
    "error": "{}",
    "onError": "{}",
    "errorContainer": "{}",
    "onErrorContainer": "{}",
    "background": "{}",
    "onBackground": "{}",
    "surface": "{}",
    "onSurface": "{}",
    "surfaceVariant": "{}",
    "onSurfaceVariant": "{}",
    "outline": "{}",
    "outlineVariant": "{}",
    "shadow": "{}",
    "scrim": "{}",
    "inverseSurface": "{}",
    "inverseOnSurface": "{}",
    "inversePrimary": "{}"
}}
"#,
        hex_from_argb(scheme.primary),
        hex_from_argb(scheme.on_primary),
        hex_from_argb(scheme.primary_container),
        hex_from_argb(scheme.on_primary_container),
        hex_from_argb(scheme.secondary),
        hex_from_argb(scheme.on_secondary),
        hex_from_argb(scheme.secondary_container),
        hex_from_argb(scheme.on_secondary_container),
        hex_from_argb(scheme.tertiary),
        hex_from_argb(scheme.on_tertiary),
        hex_from_argb(scheme.tertiary_container),
        hex_from_argb(scheme.on_tertiary_container),
        hex_from_argb(scheme.error),
        hex_from_argb(scheme.on_error),
        hex_from_argb(scheme.error_container),
        hex_from_argb(scheme.on_error_container),
        hex_from_argb(scheme.background),
        hex_from_argb(scheme.on_background),
        hex_from_argb(scheme.surface),
        hex_from_argb(scheme.on_surface),
        hex_from_argb(scheme.surface_variant),
        hex_from_argb(scheme.on_surface_variant),
        hex_from_argb(scheme.outline),
        hex_from_argb(scheme.outline_variant),
        hex_from_argb(scheme.shadow),
        hex_from_argb(scheme.scrim),
        hex_from_argb(scheme.inverse_surface),
        hex_from_argb(scheme.inverse_on_surface),
        hex_from_argb(scheme.inverse_primary)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        assert!(parse_hex_color("#FF0000").is_ok());
        assert!(parse_hex_color("#00FF00").is_ok());
        assert!(parse_hex_color("#0000FF").is_ok());
        assert!(parse_hex_color("invalid").is_err());
    }

    #[test]
    fn test_material_you_from_hex() {
        let material = MaterialYou::from_hex("#FF0000").unwrap();
        let light_theme = material.generate_theme_json("light");
        let dark_theme = material.generate_theme_json("dark");

        assert!(light_theme.contains("\"type\": \"light\""));
        assert!(dark_theme.contains("\"type\": \"dark\""));
    }
}
