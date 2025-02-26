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
