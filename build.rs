// (c) 2023 Cory Forsstrom
// (c) 2024-2025 Polesznyák Márk László

use std::path::Path;
use std::process::Command;

fn main() {
    let git_hash = Command::new("git")
        .args(["describe", "--always", "--dirty", "--exclude='*'"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|x| String::from_utf8(x.stdout).ok());

    if let Some(hash) = git_hash.as_ref() {
        println!("cargo:rustc-env=GIT_HASH={}", hash);
    }

    if git_hash.is_none() {
        return;
    }

    let Some(git_dir): Option<String> = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|x| String::from_utf8(x.stdout).ok())
    else {
        return;
    };

    let head = Path::new(&git_dir).join("HEAD");
    if head.exists() {
        println!("cargo:rerun-if-changed={}", head.display());
    }

    let Some(head_ref): Option<String> = Command::new("git")
        .args(["symbolic-ref", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|x| String::from_utf8(x.stdout).ok())
    else {
        return;
    };
    let head_ref = Path::new(&git_dir).join(head_ref);
    if head_ref.exists() {
        println!("cargo:rerun-if-changed={}", head_ref.display());
    }

    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_fontello::build("fonts/icons.toml").expect("Build icons font");
    #[cfg(windows)]
    {
        embed_resource::compile(
            "assets/windows/iced_builder.rc",
            embed_resource::NONE,
        );
        windows_exe_info::versioninfo::link_cargo_env();
    }
}
