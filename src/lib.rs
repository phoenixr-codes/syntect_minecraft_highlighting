#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::collections::HashMap;
use std::fmt::Write;
use syntect::highlighting::{Color, FontStyle, Style};

const RESET_CODE: &str = "§r";

/// The version of Minecraft.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Version {
    Java,
    Bedrock,
}

fn as_minecraft_escaped(v: &[(Style, &str)], version: Version) -> String {
    let mut s = String::new();
    for &(style, text) in v.iter() {
        let color = color_to_code(style.foreground, version);
        let style = style_to_code(style.font_style, version);
        let _ = write!(s, "{}{}{}{}", style, color, text, RESET_CODE);
    }
    s
}

/// Formats the styled fragments using Minecraft Java Edition formatting codes.
pub fn as_minecraft_java_escaped(v: &[(Style, &str)]) -> String {
    as_minecraft_escaped(v, Version::Java)
}


/// Formats the styled fragments using Minecraft Bedrock Edition formatting codes.
pub fn as_minecraft_bedrock_escaped(v: &[(Style, &str)]) -> String {
    as_minecraft_escaped(v, Version::Bedrock)
}

/// Returns the codes matching the font style as far as possible.
///
/// # References
///
/// - <https://minecraft.wiki/w/Formatting_codes>
fn style_to_code(style: FontStyle, version: Version) -> String {
    let mut s = String::new();
    if style == FontStyle::BOLD {
        s.push_str("§b");
    }
    if style == FontStyle::ITALIC {
        s.push_str("§o");
    }
    if version == Version::Java && style == FontStyle::UNDERLINE {
        s.push_str("§n");
    }
    s
}

/// Returns the code of the closest color representable in Minecraft.
///
/// # References
///
/// - <https://minecraft.wiki/w/Formatting_codes>
fn color_to_code(color: Color, version: Version) -> &'static str {
    let mut palette: HashMap<Color, &'static str> = HashMap::new();

    #[rustfmt::skip] {
        palette.insert(Color { r: 0,   g: 0,   b: 0,   a: u8::MAX }, "§0");
        palette.insert(Color { r: 0,   g: 0,   b: 170, a: u8::MAX }, "§1");
        palette.insert(Color { r: 0,   g: 170, b: 0,   a: u8::MAX }, "§2");
        palette.insert(Color { r: 0,   g: 170, b: 170, a: u8::MAX }, "§3");
        palette.insert(Color { r: 170, g: 0,   b: 0,   a: u8::MAX }, "§4");
        palette.insert(Color { r: 170, g: 0,   b: 170, a: u8::MAX }, "§5");
        palette.insert(Color { r: 255, g: 170, b: 0,   a: u8::MAX }, "§6");
        palette.insert(
            match version {
                Version::Java => Color { r: 170, g: 170, b: 170, a: u8::MAX },
                Version::Bedrock => Color { r: 198, g: 198, b: 198, a: u8::MAX },
            },
            "§7",
        );
        palette.insert(Color { r: 85,  g: 85,  b: 85,  a: u8::MAX }, "§8");
        palette.insert(Color { r: 85,  g: 85,  b: 255, a: u8::MAX }, "§9");
        palette.insert(Color { r: 85,  g: 255, b: 85,  a: u8::MAX }, "§a");
        palette.insert(Color { r: 85,  g: 255, b: 255, a: u8::MAX }, "§b");
        palette.insert(Color { r: 255, g: 85,  b: 85,  a: u8::MAX }, "§c");
        palette.insert(Color { r: 255, g: 85,  b: 255, a: u8::MAX }, "§d");
        palette.insert(Color { r: 255, g: 255, b: 85,  a: u8::MAX }, "§e");
        palette.insert(Color { r: 255, g: 255, b: 255, a: u8::MAX }, "§f");
        if version == Version::Bedrock {
            palette.insert(Color { r: 221, g: 214, b: 5,   a: u8::MAX }, "§g");
            palette.insert(Color { r: 227, g: 212, b: 209, a: u8::MAX }, "§h");
            palette.insert(Color { r: 206, g: 202, b: 202, a: u8::MAX }, "§i");
            palette.insert(Color { r: 68,  g: 58,  b: 59,  a: u8::MAX }, "§j");
            palette.insert(Color { r: 151, g: 22,  b: 7,   a: u8::MAX }, "§m");
            palette.insert(Color { r: 180, g: 104, b: 77,  a: u8::MAX }, "§n");
            palette.insert(Color { r: 222, g: 177, b: 45,  a: u8::MAX }, "§p");
            palette.insert(Color { r: 17,  g: 160, b: 54,  a: u8::MAX }, "§q");
            palette.insert(Color { r: 44,  g: 186, b: 186, a: u8::MAX }, "§s");
            palette.insert(Color { r: 33,  g: 73,  b: 123, a: u8::MAX }, "§t");
            palette.insert(Color { r: 154, g: 92,  b: 198, a: u8::MAX }, "§u");
            palette.insert(Color { r: 235, g: 114, b: 20,  a: u8::MAX }, "§v");
        }
    };

    let mut least_distance: Option<(f64, &str)> = None;

    for (key, value) in palette {
        if key == color {
            return value;
        }

        let distance: f64 = f64::sqrt(
            ((u8::abs_diff(color.r, key.r) as u32).saturating_pow(2)
                + (u8::abs_diff(color.g, key.g) as u32).pow(2)
                + (u8::abs_diff(color.b, key.b) as u32).pow(2)
                + (u8::abs_diff(color.a, key.a) as u32).pow(2)) as f64,
        );

        if least_distance.is_none_or(|(x, _)| x > distance) {
            least_distance = Some((distance, value));
        }
    }

    least_distance.unwrap().1
}
