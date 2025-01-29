use std::io;
use std::path::{Path, PathBuf};
use std::fs;

use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        let src_path = entry.path();
        let dst_path = dst.as_ref().join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn create_html_examles(src: impl AsRef<Path>, dst: impl AsRef<Path>,themeStr: &str)-> io::Result<()> {
    fs::create_dir_all(&dst)?;

    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = ss.find_syntax_by_extension("rs");
    let ts = ThemeSet::load_defaults();
    let theme= &ts.themes[themeStr];

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        let src_path = entry.path();
        let file_name = format!("{}.html",entry.file_name().into_string().unwrap());
        let dst_path = dst.as_ref().join(file_name);

        if !ty.is_dir() {
            let code = fs::read_to_string(src_path)?;
            let hl = highlighted_html_for_string(code.clone().as_str(), &ss, &(syntax.unwrap()), theme);
            let output = format!("<!DOCTYPE html><html><head><body>{}</body></html>",hl.unwrap());
            fs::write(dst_path,output)?;
        }
    }
    Ok(())
}


fn main() {
    leptos_bulma::build("./style");
    let src_dir = Path::new("src/examples");
    let dst_dir = Path::new("target/site/highlight/light");
    let _ = create_html_examles(src_dir, dst_dir,"base16-ocean.light");

    let dst_dir = Path::new("target/site/highlight/dark");
    let _ = create_html_examles(src_dir, dst_dir,"base16-ocean.dark");

}
