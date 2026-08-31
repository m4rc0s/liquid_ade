use std::process::Command;

fn main() {
    // Only re-run React build if source files change
    println!("cargo:rerun-if-changed=ui/src");
    println!("cargo:rerun-if-changed=ui/package.json");
    println!("cargo:rerun-if-changed=ui/vite.config.ts");

    println!("Building React Frontend (Vite)...");

    let install_status = Command::new("bun")
        .current_dir("ui")
        .arg("install")
        .status()
        .expect("Failed to run 'bun install'");

    assert!(install_status.success(), "UI 'bun install' failed");

    let build_status = Command::new("bun")
        .current_dir("ui")
        .arg("run")
        .arg("build")
        .status()
        .expect("Failed to run 'bun run build'");

    assert!(build_status.success(), "UI build failed");
}
