use materugen::{ MaterialYou, error::Error };
use rand::Rng;
use std::path::Path;

fn generate_random_hex_color() -> String {
    let mut t_rng = rand::thread_rng();
    format!(
        "#{:02X}{:02X}{:02X}",
        t_rng.gen_range(0..=255),
        t_rng.gen_range(0..=255),
        t_rng.gen_range(0..=255)
    )
}

fn print_help() {
    println!("materugen - Material You color scheme generator");
    println!("\nGenerate Material You color schemes from images, hex colors, or random colors");
    println!("\nUSAGE:");
    println!("    materugen <image_path> <output_json> [theme_variant]");
    println!("    materugen --hex <color> <output_json> [theme_variant]");
    println!("    materugen --random <output_json> [theme_variant]");
    println!("\nARGUMENTS:");
    println!("    <image_path>     Path to the source image (supports RGB/RGBA formats)");
    println!("    <color>          Hex color code (e.g., #FF0000, #00FF00, #0000FF)");
    println!("    <output_json>    Path for the output JSON file");
    println!("    [theme_variant]  Theme variant: 'light' or 'dark' (default: dark)");
    println!("\nOPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    --hex           Use hex color instead of image");
    println!("    --random        Generate theme from random color");
    println!("\nEXAMPLES:");
    println!("    materugen image.jpg theme.json");
    println!("    materugen image.png light.json light");
    println!("    materugen --hex \"#FF0000\" red.json");
    println!("    materugen --random random.json dark");
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 || args.get(1).map_or(false, |arg| (arg == "--help" || arg == "-h")) {
        print_help();
        return Ok(());
    }

    if args.len() < 3 {
        print_help();
        return Err(Error::Custom("Not enough arguments".to_string()));
    }

    let theme_variant = args.get(4).or(args.get(3)).map(String::as_str).unwrap_or("dark");

    match args[1].as_str() {
        "--random" => {
            let out_json = &args[2];
            let hex_color = generate_random_hex_color();
            println!("Generated random color: {}", hex_color);

            let material = MaterialYou::from_hex(&hex_color)?;
            let json = material.generate_theme_json(theme_variant);
            std::fs::write(out_json, json)?;
        }
        "--hex" => {
            if args.len() < 4 {
                return Err(Error::Custom("Missing hex color argument".to_string()));
            }
            let hex_color = &args[2];
            let out_json = &args[3];

            let material = MaterialYou::from_hex(hex_color)?;
            let json = material.generate_theme_json(theme_variant);
            std::fs::write(out_json, json)?;
        }
        image_path => {
            let out_json = &args[2];
            let image = image::open(Path::new(image_path))?;

            let material = MaterialYou::from_image(image)?;
            let json = material.generate_theme_json(theme_variant);
            std::fs::write(out_json, json)?;
        }
    }

    Ok(())
}
