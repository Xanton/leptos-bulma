use std::io;
use std::path::{Path, PathBuf};
use std::fs;

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


fn main() {
    leptos_bulma::build("./style");
    let src_dir = Path::new("js/");
    let dst_dir = Path::new("target/site/highlight/");

    let _ = copy_dir_all(src_dir, dst_dir);
}
