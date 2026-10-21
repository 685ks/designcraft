//! Optional craft-fonts build input (https://github.com/685ks/craft-fonts), embedded by its
//! `craft-fonts-build` helper (see its `docs/integration.md`). With `CRAFT_FONTS_DIR=<checkout>`
//! the fonts in its `fonts/manifest.txt` are embedded as `CRAFT_FONTS`; unset, `CRAFT_FONTS` is
//! empty. Web (wasm32) builds embed only the UI face, BIZ UDPGothic Regular (the helper's
//! default), to stay within the web size budget. The helper's `legacy` output fills this crate's
//! own `CraftFont` struct (`src/lib.rs`). It only reads the local checkout: no network.

fn main() {
    craft_fonts_build::Builder::new().legacy(true).emit();
}
