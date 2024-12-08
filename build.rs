use std::{ffi::OsStr, path::{Path, PathBuf}};


fn run_plueprint<'a, I, P>(output_dir: &impl AsRef<OsStr>, input_dir: &impl AsRef<OsStr>, filenames: I)
    where I: IntoIterator<Item = &'a P>,
          P: AsRef<Path> + ?Sized + 'a
    {

    let input_dir = Path::new(input_dir);
    let input_files: Vec<PathBuf> = filenames.into_iter().map(|f| input_dir.join(f)).collect();
    for f in &input_files {
        println!("cargo:rerun-if-changed={}", f.strip_prefix(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).unwrap().display())
    }
    std::process::Command::new("blueprint-compiler")
        .arg("batch-compile")
        .arg(output_dir)
        .arg(input_dir)
        .args(input_files)
        .spawn()
        .expect("Failed to spawn blueprint compiler")
        .wait()
        .expect("Failed to compile blueprint file");
}

fn main() {
    let cwd = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let cwd = Path::new(&cwd);
    let resources = &cwd.join("resources");
    run_plueprint(resources, resources, ["window.blp", "filters.blp"]);
    glib_build_tools::compile_resources(
        &["resources"],
          "resources/data.gresource.xml",
          "data.gresource"
    );
}
