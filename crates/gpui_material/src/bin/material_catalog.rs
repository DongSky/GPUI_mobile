use gpui_material::catalog;
use gpui_material::theme::Theme;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut args = env::args().skip(1);
    let out = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/material-catalog-light.html"));
    let dark = args.any(|a| a == "--dark");
    let theme = if dark { Theme::dark() } else { Theme::light() };
    if let Some(parent) = out.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&out, catalog::render_html(&theme)).expect("write catalog html");
    eprintln!("wrote {}", out.display());
}
