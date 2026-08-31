use std::process::Command;

fn main() {
    // Só re-executa a compilação do React se os arquivos fonte mudarem
    println!("cargo:rerun-if-changed=ui/src");
    println!("cargo:rerun-if-changed=ui/package.json");
    println!("cargo:rerun-if-changed=ui/vite.config.ts");

    println!("Construindo o Frontend React (Vite)...");

    let install_status = Command::new("bun")
        .current_dir("ui")
        .arg("install")
        .status()
        .expect("Falha ao rodar 'bun install'");

    assert!(install_status.success(), "Falha no 'bun install' da UI");

    let build_status = Command::new("bun")
        .current_dir("ui")
        .arg("run")
        .arg("build")
        .status()
        .expect("Falha ao rodar 'bun run build'");

    assert!(build_status.success(), "Falha no build da UI");
}
