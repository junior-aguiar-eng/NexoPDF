//! Embeds the attributed fonts in `assets/fonts/international` as `CRAFT_FONTS`.
//! It only reads this repository (no network). On wasm32 it embeds only the UI faces (BIZ UDPGothic Regular
//! and any `Arab`, `Telu`, `Hebr` or `Thai` face), to keep the web build within hosting limits (Cloudflare Pages:
//! 25 MiB per file).

use std::fmt::Write as _;
use std::path::PathBuf;

fn main() {
    let mut src = String::from("pub static CRAFT_FONTS: &[CraftFont] = &[\n");
    if let Some(crate_dir) = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from) {
        let dir = crate_dir.join("../../assets/fonts/international");
        match craft_fonts(&dir) {
            Ok(entries) => src.push_str(&entries),
            Err(e) => println!("cargo::error=NexoPDF local fonts: {e}"),
        }
    } else {
        println!("cargo::error=CARGO_MANIFEST_DIR is missing; cannot locate NexoPDF fonts");
    }
    src.push_str("];\n");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("craft_fonts.rs");
    if let Err(e) = std::fs::write(&out, src) {
        println!("cargo::error=writing {}: {e}", out.display());
    }
}

/// Family-name prefixes of the faces AGENTS.md §1.1 forbids: Adobe's Source families and Noto
/// CJK, which is Source Han under another name. The test in `src/craft.rs` has the same list.
const BARRED_FAMILIES: [&str; 5] = ["Source Han", "Source Serif", "Source Sans", "Noto Sans CJK", "Noto Serif CJK"];

/// One `CraftFont { .. }` initialiser per manifest line.
fn craft_fonts(dir: &std::path::Path) -> Result<String, String> {
    let manifest = dir.join("manifest.txt");
    println!("cargo::rerun-if-changed={}", manifest.display());
    let text = std::fs::read_to_string(&manifest).map_err(|e| format!("{}: {e}", manifest.display()))?;
    let wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|a| a == "wasm32");
    let mut out = String::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let f: Vec<&str> = line.split(" | ").map(str::trim).collect();
        let [family, style, file, scripts, ..] = f.as_slice() else {
            return Err(format!("malformed manifest line: {line}"));
        };
        // AGENTS.md §1.1 bars Adobe's type designs whatever their licence, so a checkout that
        // carries one never embeds it.
        if BARRED_FAMILIES.iter().any(|barred| family.starts_with(barred)) {
            return Err(format!("forbidden font family {family} {style} (AGENTS.md §1.1)"));
        }
        // Arabic interface faces are small, so the web build keeps them too.
        let arabic = scripts.split(',').any(|s| s.trim() == "Arab");
        // Telugu faces too, for the Telugu interface.
        let telugu = scripts.split(',').any(|s| s.trim() == "Telu");
        // And the Hebrew ones (small), for Hebrew text and Edit text on the web.
        let hebrew = scripts.split(',').any(|s| s.trim() == "Hebr");
        let web_face =
            (*family == "BIZ UDPGothic" && *style == "Regular") || arabic || telugu || hebrew || scripts.split(',').any(|s| s.trim() == "Thai");
        if wasm && !web_face {
            continue;
        }
        let path = dir.join(file).canonicalize().map_err(|e| format!("{file}: {e}"))?;
        println!("cargo::rerun-if-changed={}", path.display());
        let scripts: Vec<String> = scripts.split(',').map(|s| format!("{:?}", s.trim())).collect();
        let _ = writeln!(
            out,
            "    CraftFont {{ family: {family:?}, style: {style:?}, scripts: &[{}], bytes: include_bytes!({:?}) }},",
            scripts.join(", "),
            path.display().to_string(),
        );
    }
    Ok(out)
}
