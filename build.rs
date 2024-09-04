use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=assets");

    std::fs::remove_dir_all("build").unwrap_or_default();

    Command::new("npm")
        .args([
            "run",
            "build-css",
        ])
        .status()
        .expect("failed to run tailwindcss");
}
