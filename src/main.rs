mod types;
use types::*;
// use https://crates.io/crates/wkhtmltopdf

fn main() {
    let resume: Resume = toml::from_str(r#""#).unwrap();
    dbg!(resume);
}
