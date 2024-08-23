use std::env;
use std::process::Command;

fn main() {
    if env::var("SKIP_BUILD_SCRIPT").unwrap_or("0".to_string()) == "1" {
        return;
    }
    
    println!("cargo:rerun-if-changed=ui");
    
    let status = Command::new("pnpm")
        .arg("build")
        .current_dir("ui")
        .status()
        .expect("Failed to execute pnpm build");
    
    if !status.success() {
        panic!("pnpm build failed");
    }
}