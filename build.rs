use std::env;
use std::path::PathBuf;
use std::process::Command;

const CARGO_PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const VENDORS_DIR: &str = "vendors";
fn main() {
    // let mut out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR environment variable");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get CARGO_MANIFEST_DIR environment variable");

    let mut source_dir: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    dbg!(&out_dir);
    dbg!(&manifest_dir);
    dbg!(&source_dir);

    let to_look_for = format!("{}-{}", CARGO_PACKAGE_NAME, CARGO_PACKAGE_VERSION);

    for path in PathBuf::from(&manifest_dir).ancestors().into_iter() {
        if path.file_name().is_some() {
            let file_name = path.file_name().unwrap();
            if file_name == CARGO_PACKAGE_NAME {
                source_dir = Some(path.into());
                dbg!(&path.file_name());
                dbg!(&source_dir);
            } else if file_name == to_look_for.as_str() {
                out_dir = Some(path.into());
                dbg!(&path.file_name());
                dbg!(&source_dir);
            }
        }
    }
    dbg!(&out_dir);
    dbg!(&manifest_dir);
    dbg!(&source_dir);

    let source_dir = source_dir.unwrap().join(VENDORS_DIR);
    let out_dir = out_dir.unwrap_or(
        env::var("OUT_DIR")
            .expect("Failed to get OUT_DIR environment variable")
            .into(),
    );

    let dest_dir = out_dir.clone();

    dbg!(&dest_dir);
    dbg!(&out_dir);
    dbg!(&manifest_dir);
    dbg!(&source_dir);

    let _ = std::fs::create_dir_all(&dest_dir);

    let status = Command::new("cp")
        .arg("-r")
        .arg(source_dir)
        .arg(&dest_dir)
        .status()
        .expect("Failed to execute copy command");
    dbg!(&status);
    // panic!();
    if !status.success() {
        panic!("Failed to copy 'vendors/rsc' to '{}'", dest_dir.display());
    } else {
        println!(
            "Successfully moved 'vendors/rsc' to '{}'",
            dest_dir.display()
        );
    }
}
