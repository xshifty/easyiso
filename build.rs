use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=assets");

    std::fs::remove_dir_all("build").unwrap_or_default();

    Command::new("bun")
        .args([
            "run",
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "assets/css/main.css",
            "-o",
            "static/css/style.css",
            //"--minify",
        ])
        .status()
        .expect("failed to run tailwindcss");
}
