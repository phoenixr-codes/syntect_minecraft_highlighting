use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect_minecraft_highlighting::as_minecraft_bedrock_escaped;

fn main() {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 { 42 }\n";

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.light"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_minecraft_bedrock_escaped(&ranges[..]);
        println!("{}", escaped);
    }
}
