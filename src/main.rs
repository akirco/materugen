use color_thief::{get_palette, ColorFormat};
use image::DynamicImage;
use material_you_palette::utils::{string::hex_from_argb, theme::Theme};

use crate::error::Error;
mod error;

fn main() -> Result<(), error::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        return Err(Error(String::from("Not enough arguments")));
    }

    let image_path = &args[1];
    let out_json = &args[2];
    let theme_variant = if args.len() >= 4 { &args[3] } else { "dark" };

    let image = image::open(image_path)?;
    let (buffer, colorformat) = match image {
        DynamicImage::ImageRgb8(buf) => (buf.to_vec(), ColorFormat::Rgb),
        DynamicImage::ImageRgba8(buf) => (buf.to_vec(), ColorFormat::Argb),
        _ => panic!("Unknown image format"),
    };
    let color_palette = get_palette(&buffer, colorformat, 8, 2)?;
    let primary_color = color_palette[0];

    let theme = Theme::from_source_color([255, primary_color.r, primary_color.g, primary_color.b]);

    let primary_scheme = if theme_variant == "dark" {
        theme.schemes.dark
    } else {
        theme.schemes.light
    };

    let json = format!(
        r#"
{{
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
        hex_from_argb(primary_scheme.primary),
        hex_from_argb(primary_scheme.on_primary),
        hex_from_argb(primary_scheme.primary_container),
        hex_from_argb(primary_scheme.on_primary_container),
        hex_from_argb(primary_scheme.secondary),
        hex_from_argb(primary_scheme.on_secondary),
        hex_from_argb(primary_scheme.secondary_container),
        hex_from_argb(primary_scheme.on_secondary_container),
        hex_from_argb(primary_scheme.tertiary),
        hex_from_argb(primary_scheme.on_tertiary),
        hex_from_argb(primary_scheme.tertiary_container),
        hex_from_argb(primary_scheme.on_tertiary_container),
        hex_from_argb(primary_scheme.error),
        hex_from_argb(primary_scheme.on_error),
        hex_from_argb(primary_scheme.error_container),
        hex_from_argb(primary_scheme.on_error_container),
        hex_from_argb(primary_scheme.background),
        hex_from_argb(primary_scheme.on_background),
        hex_from_argb(primary_scheme.surface),
        hex_from_argb(primary_scheme.on_surface),
        hex_from_argb(primary_scheme.surface_variant),
        hex_from_argb(primary_scheme.on_surface_variant),
        hex_from_argb(primary_scheme.outline),
        hex_from_argb(primary_scheme.outline_variant),
        hex_from_argb(primary_scheme.shadow),
        hex_from_argb(primary_scheme.scrim),
        hex_from_argb(primary_scheme.inverse_surface),
        hex_from_argb(primary_scheme.inverse_on_surface),
        hex_from_argb(primary_scheme.inverse_primary)
    );

    std::fs::write(out_json, json)?;
    Ok(())
}
